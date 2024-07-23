use std::ops::Deref;
use std::sync::Arc;
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
    /// use std::sync::Arc;
    /// use voiceflousion::core::session_wrappers::LockedSession;
    /// use voiceflousion::core::session_wrappers::Session;
    ///
    /// let session = Arc::new(Session::new("chat_id".to_string(), Some(1627554661), true));
    /// let locked_session = LockedSession::try_from_session(&session);
    /// ```
    pub fn try_from_session(session: &'g Arc<Session>) -> Result<Self, VoiceflousionError> {
        let guard = session.try_lock()?;

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
    /// use std::sync::Arc;
    /// use voiceflousion::core::session_wrappers::LockedSession;
    /// use voiceflousion::core::session_wrappers::Session;    ///
    /// use voiceflousion::core::subtypes::SentMessage;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let session = Arc::new(Session::new("chat_id".to_string(), Some(1627554661), true));
    ///     let locked_session = LockedSession::try_from_session(&session);
    ///     if let Ok(locked_session) = locked_session{
    ///         let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    ///         let sent_message = SentMessage::new(block, "message_id".to_string(), 1627554661);
    ///         locked_session.set_previous_message(Some(sent_message)).await;
    ///     }
    ///
    ///     Ok(())
    /// }
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
    /// use std::sync::Arc;
    /// use chrono::Utc;
    /// use voiceflousion::core::session_wrappers::LockedSession;
    /// use voiceflousion::core::session_wrappers::Session;
    ///
    /// let session = Arc::new(Session::new("chat_id".to_string(), Some(1627554661), true));
    /// let locked_session = LockedSession::try_from_session(&session);
    /// if let Ok(locked_session) = locked_session{
    ///     locked_session.set_last_interaction(Some(Utc::now().timestamp()));
    /// }
    /// ```
    pub fn set_last_interaction(&self, last_interaction: Option<i64>) {
        self.store_last_interaction(last_interaction)
    }
}
