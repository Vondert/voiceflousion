use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use chrono::Utc;
use tokio::sync::RwLock;
use crate::integrations::utils::SessionWrapper;
use crate::integrations::utils::traits::{Session};

pub struct SessionMap<T: Session>{
    sessions: Arc<RwLock<HashMap<String, Arc<SessionWrapper<T>>>>>,
    valid_session_duration: Option<i64>
}
impl<T: Session> Deref for SessionMap<T>{
    type Target = RwLock<HashMap<String, Arc<SessionWrapper<T>>>>;

    fn deref(&self) -> &Self::Target {
        &self.sessions
    }
}
impl<T: Session> SessionMap<T>{
    pub fn new(valid_session_duration: Option<i64>) -> Self {
        Self{
            sessions: Arc::new(RwLock::new(HashMap::<String, Arc<SessionWrapper<T>>>::new())),
            valid_session_duration
        }
    }
    pub fn valid_session_duration(&self) -> &Option<i64>{
        &self.valid_session_duration
    }
    pub fn from (sessions_vec: Option<Vec<T>>, valid_session_duration: Option<i64>) -> Self{
        match sessions_vec{
            Some(vec) =>{
                let mut hash_map = HashMap::<String, Arc<SessionWrapper<T>>>::new();
                let _ = vec.into_iter().map(|session| hash_map.insert(session.get_cloned_chat_id(), Arc::new(SessionWrapper::new(session))));
                Self{
                    sessions: Arc::new(RwLock::new(hash_map)),
                    valid_session_duration
                }
            },
            None => Self::new(valid_session_duration)
        }
    }
    pub async fn get_or_add_session(&self, chat_id: &String) ->  Arc<SessionWrapper<T>> {
        if let Some(session) = self.get_session(chat_id).await{
            session
        }
        else{
           self.add_session(chat_id.clone()).await
        }
    }
    pub async fn get_session(&self, chat_id: &String) -> Option<Arc<SessionWrapper<T>>> {
        let read_lock = self.sessions.read().await;
        if let Some(session) = read_lock.get(chat_id) {
            if self.is_valid_session(session).await {
                return Some(session.clone())
            }
        }
        None
    }
    pub async fn add_session(&self, chat_id: String) -> Arc<SessionWrapper<T>> {
        let mut write_lock = self.sessions.write().await;
        let session = Arc::new(SessionWrapper::new(T::from_chat_id(chat_id.clone(), None)));
        write_lock.insert(chat_id.clone(), session.clone());
        session
    }
    async fn is_valid_session(&self, session: &Arc<SessionWrapper<T>>) -> bool{
        if let Some(duration) = &self.valid_session_duration{
            let now = Utc::now().timestamp();
            if let Some(last_interaction) = session.get_last_interaction().await{
                !(now - last_interaction > *duration)
            }
            else{
                false
            }
        }
        else{
            true
        }
    }
}