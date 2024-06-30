use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::integrations::Session;

pub(crate) struct SessionMap<T: Session>{
    sessions: Arc<RwLock<HashMap<String, Arc<T>>>>,
}
impl<T: Session> Deref for SessionMap<T>{
    type Target = RwLock<HashMap<String, Arc<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.sessions
    }
}
impl<T: Session> SessionMap<T>{
    pub fn new() -> Self {
        Self{
            sessions: Arc::new(RwLock::new(HashMap::<String, Arc<T>>::new()))
        }
    }
    pub fn from (sessions_vec: Option<Vec<T>>) -> Self{
        match sessions_vec{
            Some(vec) =>{
                let mut hash_map = HashMap::<String, Arc<T>>::new();
                let _ = vec.into_iter().map(|session| hash_map.insert(session.get_cloned_chat_id(), Arc::new(session)));
                Self{
                    sessions: Arc::new(RwLock::new(hash_map))
                }
            },
            None => Self::new()
        }
    }
    pub async fn get_or_add_session_async(&self, chat_id: String) -> Arc<T> {
        let read_lock = self.sessions.read().await;
        let session = if let Some(session) = read_lock.get(&chat_id) {
            Arc::clone(session)
        }
        else{
            drop(read_lock);
            let mut write_lock = self.sessions.write().await;
            let session = write_lock.entry(chat_id.clone())
                .or_insert_with(|| Arc::new(T::from_chat_id(chat_id.clone())))
                .clone();
            session
        };
        session
    }
}