use tokio::sync::MutexGuard;
use crate::voiceflow::VoiceflowError;

pub(crate) trait Session{
    fn from_chat_id(chat_id: String) -> Self;
    fn get_chat_id(&self) -> &String;
    fn get_cloned_chat_id(&self) -> String;
    fn try_lock(&self) -> Result<MutexGuard<bool>, VoiceflowError>;
}