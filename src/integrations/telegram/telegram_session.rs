use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::{Mutex, RwLock};
use crate::integrations::utils::traits::{Session, SessionBase};
use crate::voiceflow::request_structures::VoiceflowSession;


pub struct TelegramSession{
    chat_id: String,
    voiceflow_session: VoiceflowSession,
    lock: Arc<Mutex<bool>>,
    status: Arc<RwLock<bool>>,
    last_interaction: Arc<RwLock<Option<i64>>>
}
impl Deref for TelegramSession{
    type Target = VoiceflowSession;
    fn deref(&self) -> &Self::Target {
        &self.voiceflow_session
    }
}
impl TelegramSession{
    fn new (chat_id: String, voiceflow_session: VoiceflowSession, last_interaction: Option<i64>) -> Self{
        Self{
            chat_id,
            voiceflow_session,
            lock: Arc::new(Mutex::new(true)),
            status: Arc::new(RwLock::new(true)),
            last_interaction: Arc::new(RwLock::new(last_interaction))
        }
    }
}
impl SessionBase for TelegramSession{
    fn from_chat_id(chat_id: String, interaction: Option<i64>) -> Self{
        let voiceflow_session = VoiceflowSession::from_chat_id(&chat_id);
        Self::new(chat_id, voiceflow_session, interaction)
    }

    fn get_chat_id(&self) -> &String {
        &self.chat_id
    }

    fn get_cloned_chat_id(&self) -> String {
        self.chat_id.clone()
    }

    fn get_lock(&self) -> &Arc<Mutex<bool>> {
        &self.lock
    }

    fn last_interaction(&self) -> &Arc<RwLock<Option<i64>>> {
        &self.last_interaction
    }

    fn status(&self) -> &Arc<RwLock<bool>> {
        &self.status
    }
}
#[async_trait]
impl Session for TelegramSession{}