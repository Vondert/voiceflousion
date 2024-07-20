use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::integrations::utils::session_wrappers::{Session, SessionMap};

pub struct SessionsManager{
    session_map: Arc<SessionMap>,
    cancel_token: Option<Arc<AtomicBool>>,
}
impl Deref for SessionsManager{
    type Target = Arc<SessionMap>;

    fn deref(&self) -> &Self::Target {
        &self.session_map
    }
}
impl SessionsManager{
    pub fn new(sessions_option: Option<Vec<Session>>, valid_session_duration: Option<i64>, cleanup_interval: Option<u64>, is_cleaning: bool) -> Self{
        let manager = Self{
            session_map: Arc::new(
                match sessions_option{
                    None =>  SessionMap::new(
                        valid_session_duration,
                        cleanup_interval
                    ),
                    Some(sessions) => SessionMap::from_sessions(
                        sessions,
                        valid_session_duration,
                        cleanup_interval
                    )
                }
            ),
            cancel_token: if is_cleaning {
                Some(Arc::new(AtomicBool::new(false)))
            }
            else{
                None
            }
        };

        let sessions_map = manager.session_map.clone();
        if let Some(token) = manager.cancel_token.clone(){
            let cancel_token = token.clone();
            tokio::spawn(async move {
                sessions_map.start_cleanup(cancel_token).await
            });
        }

        manager
    }
}
impl Drop for SessionsManager {
    fn drop(&mut self) {
        if let Some(token) = &mut self.cancel_token{
            token.store(true, Ordering::Release);
        }
    }
}