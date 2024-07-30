use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use tokio::sync::RwLock;
use crate::core::session_wrappers::Session;

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
    pub(crate) fn from_sessions(sessions_vec: Vec<Session>, valid_session_duration: Option<i64>) -> Self {
        let mut hash_map = HashMap::<String, Arc<Session>>::new();
        sessions_vec.into_iter().for_each(|session| {
            hash_map.insert(session.get_cloned_chat_id(), Arc::new(session));
        });
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
    pub async fn delete_session(&self, chat_id: &String) {
        let mut write_lock = self.sessions.write().await;
        write_lock.remove(chat_id);
    }

    /// Deletes all invalid sessions.
    pub(crate) async fn delete_invalid_sessions(&self) {
        let mut write_lock = self.sessions.write().await;
        let keys: Vec<String> = write_lock.keys().cloned().collect();
        for key in keys {
            let is_delete = if let Some(session) = write_lock.get(&key) {
                let is_invalid = !self.is_valid_session(session).await;
                let can_lock = session.try_lock().is_ok();
                is_invalid && can_lock
            }
            else{
                false
            };
            if is_delete{
                write_lock.remove(&key);
            }
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
