use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::integrations::utils::subtypes::{AtomicTimestamp, SentMessage};
use crate::voiceflow::VoiceflowSession;

/// Represents a session for handling interactions.
///
/// `Session` manages the state and interaction details for a chat session.
pub struct Session {
    /// The chat ID associated with the session.
    chat_id: String,
    /// The status of the session (active/inactive).
    status: Arc<AtomicBool>,
    /// The timestamp of the last interaction.
    pub(super) last_interaction: Arc<AtomicTimestamp>,
    /// The previous message sent in the session.
    previous_message: Arc<RwLock<Option<SentMessage>>>,
    /// The Voiceflow session associated with the session.
    voiceflow_session: VoiceflowSession,
    /// The lock for managing session concurrency.
    pub(super) lock: Arc<Mutex<bool>>,
}

impl Session {
    /// Creates a new session.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID associated with the session.
    /// * `last_interaction` - The optional timestamp of the last interaction.
    /// * `status` - The initial status of the session.
    ///
    /// # Returns
    ///
    /// A new instance of `Session`.
    ///
    /// # Example
    ///
    /// ```
    /// let session = Session::new("chat_id".to_string(), Some(1627554661), true);
    /// ```
    pub fn new(chat_id: String, last_interaction: Option<i64>, status: bool) -> Self {
        let voiceflow_session = VoiceflowSession::from_chat_id(&chat_id);
        Self {
            chat_id,
            voiceflow_session,
            status: Arc::new(AtomicBool::new(status)),
            last_interaction: Arc::new(AtomicTimestamp::new(last_interaction)),
            previous_message: Arc::new(RwLock::new(None)),
            lock: Arc::new(Mutex::new(true)),
        }
    }

    /// Returns the previous message sent in the session.
    ///
    /// # Returns
    ///
    /// An `RwLockReadGuard` to the previous message.
    ///
    /// # Example
    ///
    /// ```
    /// let previous_message = session.previous_message().await;
    /// ```
    pub async fn previous_message(&self) -> RwLockReadGuard<'_, Option<SentMessage>> {
        let binding = &self.previous_message;
        let message = binding.read().await;
        message
    }

    /// Acquires a write lock to the previous message.
    ///
    /// # Returns
    ///
    /// An `RwLockWriteGuard` to the previous message.
    ///
    /// # Example
    ///
    /// ```
    /// let mut previous_message = session.write_previous_message().await;
    /// *previous_message = Some(SentMessage::new(...));
    /// ```
    pub(super) async fn write_previous_message(&self) -> RwLockWriteGuard<'_, Option<SentMessage>> {
        let binding = &self.previous_message;
        let previous = binding.write().await;
        previous
    }

    /// Returns the timestamp of the last interaction.
    ///
    /// # Returns
    ///
    /// An `Option<i64>` containing the timestamp of the last interaction.
    ///
    /// # Example
    ///
    /// ```
    /// let last_interaction = session.get_last_interaction();
    /// ```
    pub fn get_last_interaction(&self) -> Option<i64> {
        self.last_interaction.load(Ordering::SeqCst)
    }

    /// Activates the session.
    ///
    /// # Example
    ///
    /// ```
    /// session.activate();
    /// ```
    pub fn activate(&self) {
        self.status.store(true, Ordering::Release)
    }

    /// Deactivates the session.
    ///
    /// # Example
    ///
    /// ```
    /// session.deactivate();
    /// ```
    pub fn deactivate(&self) {
        self.status.store(false, Ordering::Release)
    }

    /// Checks if the session is active.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the session is active.
    ///
    /// # Example
    ///
    /// ```
    /// let is_active = session.is_active();
    /// ```
    pub fn is_active(&self) -> bool {
        self.status.load(Ordering::Acquire)
    }

    /// Returns a reference to the chat ID.
    ///
    /// # Returns
    ///
    /// A reference to the chat ID string.
    ///
    /// # Example
    ///
    /// ```
    /// let chat_id = session.get_chat_id();
    /// ```
    pub fn get_chat_id(&self) -> &String {
        &self.chat_id
    }

    /// Returns a cloned chat ID.
    ///
    /// # Returns
    ///
    /// A cloned chat ID string.
    ///
    /// # Example
    ///
    /// ```
    /// let cloned_chat_id = session.get_cloned_chat_id();
    /// ```
    pub fn get_cloned_chat_id(&self) -> String {
        self.get_chat_id().clone()
    }

    /// Returns a reference to the Voiceflow session.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowSession`.
    ///
    /// # Example
    ///
    /// ```
    /// let vf_session = session.voiceflow_session();
    /// ```
    pub fn voiceflow_session(&self) -> &VoiceflowSession {
        &self.voiceflow_session
    }
}