use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use crate::voiceflow::VoiceflowError;

pub trait Session{
    fn from_chat_id(chat_id: String, interaction: Option<i64>) -> Self;
    fn get_chat_id(&self) -> &String;
    fn get_cloned_chat_id(&self) -> String;
    fn try_lock(&self) -> Result<MutexGuard<'_, bool>, VoiceflowError>;
    fn last_interaction(&self) -> Arc<Mutex<Option<i64>>>;
    fn get_last_interaction_locked(&self) -> Result<Option<i64>, VoiceflowError> {
        let binding = self.last_interaction();
        let last_interaction = binding.try_lock().map_err(|_| VoiceflowError::SessionLockError)?;
        Ok(*last_interaction)
    }
    fn set_last_interaction_locked(&self, interaction: i64) -> Result<(), VoiceflowError>{
        let binding = self.last_interaction();
        let mut last_interaction = binding.try_lock().map_err(|_| VoiceflowError::SessionLockError)?;
        *last_interaction = Some(interaction);
        Ok(())
    }
}