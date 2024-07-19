use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::integrations::utils::subtypes::AtomicTimestamp;
use crate::voiceflow::VoiceflowSession;

pub trait Session: Send + Sync{
    fn from_chat_id(chat_id: String, interaction: Option<i64>) -> Self;
    fn get_chat_id(&self) -> &String;
    fn get_cloned_chat_id(&self) -> String {
        self.get_chat_id().clone()
    }
    fn last_interaction(&self) -> &Arc<AtomicTimestamp>;
    fn status(&self) -> &Arc<AtomicBool>;
    fn voiceflow_session(&self) -> &VoiceflowSession;
}