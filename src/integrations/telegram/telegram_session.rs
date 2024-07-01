use std::ops::Deref;
use std::sync::Arc;
use chrono::Utc;
use tokio::sync::{Mutex, MutexGuard};
use crate::integrations::session::Session;
use crate::voiceflow::request_structures::VoiceflowSession;
use crate::voiceflow::VoiceflowError;


pub(crate) struct TelegramSession{
    chat_id: String,
    voiceflow_session: VoiceflowSession,
    lock: Arc<Mutex<bool>>,
    last_interaction: Arc<Mutex<i64>>
}
impl Deref for TelegramSession{
    type Target = VoiceflowSession;

    fn deref(&self) -> &Self::Target {
        &self.voiceflow_session
    }
}
impl TelegramSession{
    fn new (chat_id: String, voiceflow_session: VoiceflowSession, last_interaction: i64) -> Self{
        Self{
            chat_id,
            voiceflow_session,
            lock: Arc::new(Mutex::new(true)),
            last_interaction: Arc::new(Mutex::new(last_interaction))
        }
    }
}
impl Session for TelegramSession{
    fn from_chat_id(chat_id: String, interaction: Option<i64>) -> Self{
        let voiceflow_session = VoiceflowSession::from_chat_id(&chat_id);
        let interaction_time = interaction.unwrap_or_else(|| Utc::now().timestamp());
        Self::new(chat_id, voiceflow_session, interaction_time)
    }

    fn get_chat_id(&self) -> &String {
        &self.chat_id
    }

    fn get_cloned_chat_id(&self) -> String {
        self.chat_id.clone()
    }

    fn try_lock(&self) -> Result<MutexGuard<bool>, VoiceflowError> {
        self.lock.try_lock().map_err(|_| VoiceflowError::SessionLockError)
    }

    fn last_interaction(&self) -> Arc<Mutex<i64>> {
        self.last_interaction.clone()
    }
}