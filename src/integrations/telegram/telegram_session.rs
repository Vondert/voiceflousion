use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::RwLock;
use crate::integrations::utils::traits::{Session};
use crate::voiceflow::{VoiceflowSession};

pub struct TelegramSession{
    chat_id: String,
    voiceflow_session: VoiceflowSession,
    status: Arc<AtomicBool>,
    last_interaction: Arc<RwLock<Option<i64>>>,
}
impl TelegramSession{
    fn new (chat_id: String, voiceflow_session: VoiceflowSession, last_interaction: Option<i64>) -> Self{
        Self{
            chat_id,
            voiceflow_session,
            status: Arc::new(AtomicBool::new(true)),
            last_interaction: Arc::new(RwLock::new(last_interaction))
        }
    }
}
impl Session for TelegramSession{
    fn from_chat_id(chat_id: String, interaction: Option<i64>) -> Self{
        let voiceflow_session = VoiceflowSession::from_chat_id(&chat_id);
        Self::new(chat_id, voiceflow_session, interaction)
    }

    fn get_chat_id(&self) -> &String {
        &self.chat_id
    }

    fn last_interaction(&self) -> &Arc<RwLock<Option<i64>>> {
        &self.last_interaction
    }

    fn status(&self) -> &Arc<AtomicBool> {
        &self.status
    }

    fn voiceflow_session(&self) -> &VoiceflowSession {
        &self.voiceflow_session
    }
}