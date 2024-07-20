use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::Utc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use crate::integrations::utils::session_wrappers::Session;

pub struct SessionMap{
    sessions: Arc<RwLock<HashMap<String, Arc<Session>>>>,
    cleanup_interval: Option<u64>,
    valid_session_duration: Option<i64>
}
impl SessionMap {
    pub(crate) fn new(valid_session_duration: Option<i64>, cleanup_interval: Option<u64>) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::<String, Arc<Session>>::new())),
            valid_session_duration,
            cleanup_interval
        }
    }
    pub(crate) fn from_sessions(sessions_vec: Vec<Session>, valid_session_duration: Option<i64>, cleanup_interval: Option<u64>) -> Self {
        let mut hash_map = HashMap::<String, Arc<Session>>::new();
        let _ = sessions_vec.into_iter().map(|session| hash_map.insert(session.get_cloned_chat_id(), Arc::new(session)));
        Self {
            sessions: Arc::new(RwLock::new(hash_map)),
            valid_session_duration,
            cleanup_interval
        }
    }
    pub async fn get_session(&self, chat_id: &String) -> Option<Arc<Session>> {
        let read_lock = self.sessions.read().await;
        if let Some(session) = read_lock.get(chat_id) {
            if self.is_valid_session(session).await {
                return Some(session.clone())
            }
        }
        None
    }
    pub async fn add_session(&self, chat_id: String) -> Arc<Session> {
        let mut write_lock = self.sessions.write().await;
        let session = write_lock.entry(chat_id.clone())
            .or_insert_with(|| Arc::new(Session::new(chat_id, None, true)))
            .clone();
        session
    }
    pub async fn delete_session(&self, chat_id: &String) -> () {
        let mut write_lock = self.sessions.write().await;
        write_lock.remove(chat_id);
    }
    pub async fn delete_invalid_sessions(&self) -> () {
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
    pub(crate) async fn start_cleanup(&self, cancel_token: Arc<AtomicBool>) -> () {
        if let Some(seconds) = self.cleanup_interval {
            let mut interval = interval(Duration::from_secs(seconds));
            interval.tick().await;
            loop {
                interval.tick().await;
                if cancel_token.load(Ordering::Acquire) {
                    break;
                }
                self.delete_invalid_sessions().await;
            }
        }
    }
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