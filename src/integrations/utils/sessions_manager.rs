use std::ops::Deref;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use crate::integrations::utils::session_wrappers::SessionMap;
use crate::integrations::utils::traits::Session;

pub struct SessionsManager<S: Session>{
    session_map: Arc<SessionMap<S>>,
    cancel_token: Option<Arc<CancellationToken>>,
}
impl<S: Session> Deref for SessionsManager<S>{
    type Target = Arc<SessionMap<S>>;

    fn deref(&self) -> &Self::Target {
        &self.session_map
    }
}
impl<S: Session + 'static> SessionsManager<S>{
    pub fn new(sessions_option: Option<Vec<S>>, valid_session_duration: Option<i64>, cleanup_interval: Option<u64>, is_cleaning: bool) -> Self{
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
                Some(Arc::new(CancellationToken::new()))
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
impl<S: Session> Drop for SessionsManager<S> {
    fn drop(&mut self) {
        if let Some(token) = &mut self.cancel_token{
            token.cancel();
        }
    }
}