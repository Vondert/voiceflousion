use std::ops::Deref;
use crate::voiceflow::request_structures::VoiceflowSession;

pub(crate) struct TelegramSession{
    chat_id: String,
    voiceflow_session: VoiceflowSession,
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
            voiceflow_session
        }
    }
    pub fn from_chat_id(chat_id: String) -> Self{
        let voiceflow_session = VoiceflowSession::from_chat_id(&chat_id);
        Self::new(chat_id, voiceflow_session)
    }

}