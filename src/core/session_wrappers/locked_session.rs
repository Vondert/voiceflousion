use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::MutexGuard;
use crate::core::session_wrappers::Session;
use crate::core::subtypes::SentMessage;
use crate::core::voiceflow::VoiceflousionError;

/// Represents a locked session for thread-safe operations.
///
/// `LockedSession` ensures that session operations are performed in a thread-safe manner
/// by acquiring a lock on the session.
pub struct LockedSession<'g> {
    /// A reference to the session.
    session: &'g Arc<Session>,
    /// A guard that holds the lock on the session.
    _guard: MutexGuard<'g, bool>,
}

impl<'g> Deref for LockedSession<'g> {
    type Target = Arc<Session>;

    /// Dereferences to the underlying session.
    ///
    /// # Returns
    ///
    /// A reference to the session.
    fn deref(&self) -> &'g Self::Target {
        self.session
    }
}

impl<'g> LockedSession<'g> {
    /// Tries to create a `LockedSession` from a session by acquiring a lock.
    ///
    /// # Parameters
    ///
    /// * `session` - A reference to the session.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `LockedSession` or a `VoiceflousionError` if the lock cannot be acquired.
    ///
    /// # Example
    ///
    /// ```
    /// let locked_session = LockedSession::try_from_session(&session)?;
    /// ```
    pub fn try_from_session(session: &'g Arc<Session>) -> Result<Self, VoiceflousionError> {
        let binding = &session.lock;
        let guard = binding.try_lock().map_err(|_| VoiceflousionError::SessionLockError(session.get_cloned_chat_id()))?;

        Ok(Self {
            session,
            _guard: guard,
        })
    }

    /// Sets the previous message in the session.
    ///
    /// # Parameters
    ///
    /// * `message` - The previous message to set.
    ///
    /// # Example
    ///
    /// ```
    /// locked_session.set_previous_message(Some(sent_message)).await;
    /// ```
    pub async fn set_previous_message(&self, message: Option<SentMessage>) {
        let mut write = self.session.write_previous_message().await;
        *write = message;
    }

    /// Sets the timestamp of the last interaction.
    ///
    /// # Parameters
    ///
    /// * `last_interaction` - The timestamp of the last interaction.
    ///
    /// # Example
    ///
    /// ```
    /// locked_session.set_last_interaction(Some(Utc::now().timestamp()));
    /// ```
    pub fn set_last_interaction(&self, last_interaction: Option<i64>) {
        self.last_interaction.store(last_interaction, Ordering::SeqCst)
    }
}
