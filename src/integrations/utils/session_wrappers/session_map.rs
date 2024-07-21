use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::Utc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use crate::integrations::utils::session_wrappers::Session;

/// Represents a map of sessions with cleanup functionality.
///
/// `SessionMap` manages sessions, allows adding, retrieving, and deleting sessions,
/// and provides functionality for periodic cleanup of invalid sessions.
pub struct SessionMap {
    /// The map of sessions.
    sessions: Arc<RwLock<HashMap<String, Arc<Session>>>>,
    /// The duration a session is considered valid in seconds.
    valid_session_duration: Option<i64>,
}

impl SessionMap {
    /// Creates a new `SessionMap`.
    ///
    /// # Parameters
    ///
    /// * `valid_session_duration` - The duration a session is considered valid in seconds.
    ///
    /// # Returns
    ///
    /// A new instance of `SessionMap`.
    ///
    /// # Example
    ///
    /// ```
    /// let session_map = SessionMap::new(Some(3600), Some(600));
    /// ```
    pub(crate) fn new(valid_session_duration: Option<i64>) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::<String, Arc<Session>>::new())),
            valid_session_duration
        }
    }

    /// Creates a `SessionMap` from a vector of sessions.
    ///
    /// # Parameters
    ///
    /// * `sessions_vec` - A vector of sessions to initialize the map with.
    /// * `valid_session_duration` - The duration a session is considered valid in seconds.
    ///
    /// # Returns
    ///
    /// A new instance of `SessionMap`.
    ///
    /// # Example
    ///
    /// ```
    /// let sessions_vec = vec![];
    /// let session_map = SessionMap::from_sessions(sessions_vec, Some(3600));
    /// ```
    pub(crate) fn from_sessions(sessions_vec: Vec<Session>, valid_session_duration: Option<i64>) -> Self {
        let mut hash_map = HashMap::<String, Arc<Session>>::new();
        let _ = sessions_vec.into_iter().map(|session| hash_map.insert(session.get_cloned_chat_id(), Arc::new(session)));
        Self {
            sessions: Arc::new(RwLock::new(hash_map)),
            valid_session_duration
        }
    }

    /// Returns the duration a session is considered valid.
    ///
    /// # Returns
    ///
    /// An `Option<i64>` representing the session duration in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let duration = session_map.valid_session_duration();
    /// ```
    pub fn valid_session_duration(&self) -> Option<i64>{
        self.valid_session_duration
    }

    /// Retrieves a session by chat ID.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the session to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing an `Arc` to the session if found and valid.
    ///
    /// # Example
    ///
    /// ```
    /// let session = session_map.get_session(&"chat_id".to_string()).await;
    /// ```
    pub async fn get_session(&self, chat_id: &String) -> Option<Arc<Session>> {
        let read_lock = self.sessions.read().await;
        if let Some(session) = read_lock.get(chat_id) {
            if self.is_valid_session(session).await {
                return Some(session.clone());
            }
        }
        None
    }

    /// Retrieves all sessions.
    ///
    /// # Returns
    ///
    /// A vector of `Arc<Session>` containing all sessions.
    ///
    /// # Example
    ///
    /// ```
    /// let sessions = session_map.get_all_sessions().await;
    /// ```
    pub async fn get_all_sessions(&self) -> Vec<Arc<Session>> {
        let read_lock = self.sessions.read().await;
        let sessions = read_lock.values().cloned().collect();
        sessions
    }

    /// Adds a new session by chat ID or returns the existing session.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the session to add.
    ///
    /// # Returns
    ///
    /// An `Arc` to the newly added or existing session.
    ///
    /// # Example
    ///
    /// ```
    /// let session = session_map.add_session("chat_id".to_string()).await;
    /// ```
    pub async fn add_session(&self, chat_id: String) -> Arc<Session> {
        let mut write_lock = self.sessions.write().await;
        let session = write_lock.entry(chat_id.clone())
            .or_insert_with(|| Arc::new(Session::new(chat_id, None, true)))
            .clone();
        session
    }

    /// Deletes a session by chat ID.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the session to delete.
    ///
    /// # Example
    ///
    /// ```
    /// session_map.delete_session(&"chat_id".to_string()).await;
    /// ```
    pub async fn delete_session(&self, chat_id: &String) {
        let mut write_lock = self.sessions.write().await;
        write_lock.remove(chat_id);
    }

    /// Deletes all invalid sessions.
    ///
    /// # Example
    ///
    /// ```
    /// session_map.delete_invalid_sessions().await;
    /// ```
    pub async fn delete_invalid_sessions(&self) {
        let mut write_lock = self.sessions.write().await;
        let mut sessions_to_remove = vec![];
        for (key, session) in write_lock.iter() {
            if !self.is_valid_session(session).await {
                sessions_to_remove.push(key.clone());
            }
        }
        for key in sessions_to_remove {
            write_lock.remove(&key);
        }
    }

    /// Starts the periodic cleanup of invalid sessions.
    ///
    /// # Parameters
    ///
    /// * `cleanup_interval` - The interval for cleanup in seconds.
    /// * `cancel_token` - An atomic boolean to cancel the cleanup process.
    ///
    /// # Example
    ///
    /// ```
    /// let cancel_token = Arc::new(AtomicBool::new(false));
    /// session_map.start_cleanup(600, cancel_token).await;
    /// ```
    pub(crate) async fn start_cleanup(&self, cleanup_interval: u64, cancel_token: Arc<AtomicBool>) {
        let mut interval = interval(Duration::from_secs(cleanup_interval));
        interval.tick().await;
        loop {
            interval.tick().await;
            if cancel_token.load(Ordering::Acquire) {
                break;
            }
            self.delete_invalid_sessions().await;
        }
    }

    /// Checks if a session is valid.
    ///
    /// # Parameters
    ///
    /// * `session` - A reference to the session to check.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the session is valid.
    ///
    /// # Example
    ///
    /// ```
    /// let is_valid = session_map.is_valid_session(&session).await;
    /// ```
    async fn is_valid_session(&self, session: &Arc<Session>) -> bool {
        if let Some(last_interaction) = session.get_last_interaction() {
            if let Some(duration) = &self.valid_session_duration {
                let now = Utc::now().timestamp();
                !(now - last_interaction > *duration)
            } else {
                true
            }
        } else {
            false
        }
    }
}
