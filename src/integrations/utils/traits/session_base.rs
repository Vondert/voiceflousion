use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::voiceflow::VoiceflowSession;

pub trait SessionBase: Send + Sync{
    fn from_chat_id(chat_id: String, interaction: Option<i64>) -> Self;
    fn get_chat_id(&self) -> &String;
    fn get_cloned_chat_id(&self) -> String;
    fn get_lock(&self) -> &Arc<Mutex<bool>>;
    fn last_interaction(&self) -> &Arc<RwLock<Option<i64>>>;
    fn status(&self) -> &Arc<RwLock<bool>>;
    fn voiceflow_session(&self) -> &VoiceflowSession;
}