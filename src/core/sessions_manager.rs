use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::core::session_wrappers::{Session, SessionMap};

/// Manages the sessions and handles cleanup operations.
///
/// `SessionsManager` provides functionalities for creating and managing sessions,
/// including the ability to start periodic cleanup of invalid sessions.
pub struct SessionsManager {
    /// The map of sessions.
    session_map: Arc<SessionMap>,
    /// The cancel token for stopping the cleanup process.
    cancel_token: Arc<AtomicBool>,
    /// The interval for cleanup in seconds.
    cleanup_interval: Option<u64>,
}

impl Deref for SessionsManager {
    type Target = Arc<SessionMap>;

    /// Dereferences to the underlying session map.
    ///
    /// # Returns
    ///
    /// A reference to the `SessionMap`.
    fn deref(&self) -> &Self::Target {
        &self.session_map
    }
}

impl SessionsManager {
    /// Creates a new `SessionsManager`.
    ///
    /// # Parameters
    ///
    /// * `sessions_option` - An optional vector of sessions to initialize the session map with.
    /// * `valid_session_duration` - The duration a session is considered valid in seconds.
    /// * `cleanup_interval` - The interval for cleanup in seconds.
    ///
    /// # Returns
    ///
    /// A new instance of `SessionsManager`.
    ///
    /// # Example
    ///
    /// ```
    /// let sessions_manager = SessionsManager::new(Some(sessions), Some(3600), Some(600));
    /// ```
    pub fn new(sessions_option: Option<Vec<Session>>, valid_session_duration: Option<i64>, cleanup_interval: Option<u64>) -> Self {
        let manager = Self {
            session_map: Arc::new(
                match sessions_option {
                    None => SessionMap::new(
                        valid_session_duration,
                    ),
                    Some(sessions) => SessionMap::from_sessions(
                        sessions,
                        valid_session_duration,
                    ),
                }
            ),
            cancel_token: Arc::new(AtomicBool::new(false)),
            cleanup_interval
        };

        let sessions_map = manager.session_map.clone();
        if let Some(interval) = manager.cleanup_interval.clone() {
            let cancel_token = manager.cancel_token.clone();
            tokio::spawn(async move {
                sessions_map.start_cleanup(interval, cancel_token).await;
            });
        }

        manager
    }
    pub fn cleanup_interval(&self) -> Option<u64>{
        self.cleanup_interval
    }
}

impl Drop for SessionsManager {
    /// Drops the `SessionsManager` and stops the cleanup process.
    fn drop(&mut self) {
        &mut self.cancel_token.store(true, Ordering::Release);
    }
}
