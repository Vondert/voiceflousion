use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{interval, Duration};
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
    /// use voiceflousion::core::session_wrappers::Session;
    /// use voiceflousion::core::base_structs::SessionsManager;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     let sessions: Vec<Session> = vec![];
    ///     let sessions_manager = SessionsManager::new(Some(sessions), Some(3600), Some(600));
    /// }
    /// ```
    pub fn new(sessions_option: Option<Vec<Session>>, valid_session_duration: Option<i64>, cleanup_interval: Option<u64>) -> Self {
        let manager = Self {
            session_map: Arc::new(
                match sessions_option {
                    None => SessionMap::new(valid_session_duration),
                    Some(sessions) => SessionMap::from_sessions(sessions, valid_session_duration),
                }
            ),
            cancel_token: Arc::new(AtomicBool::new(true)),
            cleanup_interval,
        };

        // Start the cleanup process if a cleanup interval is specified
        if let Some(interval) = manager.cleanup_interval.clone() {
            manager.start_cleanup(interval);
        }
        manager
    }

    /// Returns the cleanup interval.
    ///
    /// # Returns
    ///
    /// An `Option<u64>` representing the interval for sessions cleanup in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::session_wrappers::Session;
    /// use voiceflousion::core::base_structs::SessionsManager;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     let sessions: Vec<Session> = vec![];
    ///     let sessions_manager = SessionsManager::new(Some(sessions), Some(3600), Some(600));
    ///
    ///     let interval = sessions_manager.cleanup_interval();
    /// }
    /// ```
    pub fn cleanup_interval(&self) -> Option<u64> {
        self.cleanup_interval
    }

    /// Starts the sessions cleanup process if it is not already running.
    ///
    /// # Parameters
    ///
    /// * `cleanup_interval` - The interval for cleanup in seconds.
    pub(super) fn start_cleanup(&self, cleanup_interval: u64) {
        if self.cancel_token.load(Ordering::Acquire) {
            self.cancel_token.store(false, Ordering::Release);

            let cancel_token = self.cancel_token.clone();
            let sessions_map = self.session_map.clone();

            // Spawn a new asynchronous task to periodically clean up invalid sessions
            tokio::spawn(async move {
                let mut cleanup_interval = interval(Duration::from_secs(cleanup_interval));
                cleanup_interval.tick().await;
                loop {
                    cleanup_interval.tick().await;
                    if cancel_token.load(Ordering::Acquire) {
                        break;
                    }
                    sessions_map.delete_invalid_sessions().await;
                }
            });
        }
    }

    /// Stops the sessions cleanup process.
    pub(super) fn stop_cleanup(&self) {
        if !self.cancel_token.load(Ordering::Acquire) {
            self.cancel_token.store(true, Ordering::Release);
        }
    }
}

impl Drop for SessionsManager {
    /// Drops the `SessionsManager` and stops the cleanup process.
    fn drop(&mut self) {
        self.stop_cleanup();
    }
}
