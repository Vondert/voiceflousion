use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use crate::integrations::session::Session;
use crate::voiceflow::request_structures::VoiceflowSession;
use crate::voiceflow::VoiceflowError;


pub(crate) struct TelegramSession{
    chat_id: String,
    voiceflow_session: VoiceflowSession,
    lock: Arc<Mutex<bool>>
}
impl Deref for TelegramSession{
    type Target = VoiceflowSession;

    fn deref(&self) -> &Self::Target {
        &self.voiceflow_session
    }
}
impl TelegramSession{
    fn new (chat_id: String, voiceflow_session: VoiceflowSession) -> Self{
        Self{
            chat_id,
            voiceflow_session,
            lock: Arc::new(Mutex::new(true))
        }
    }
}
impl Session for TelegramSession{
    fn from_chat_id(chat_id: String) -> Self{
        let voiceflow_session = VoiceflowSession::from_chat_id(&chat_id);
        Self::new(chat_id, voiceflow_session)
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
}