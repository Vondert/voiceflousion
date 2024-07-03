use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::{MutexGuard, RwLock};
use crate::voiceflow::VoiceflowError;

#[async_trait]
pub trait Session{
    fn from_chat_id(chat_id: String, interaction: Option<i64>) -> Self;
    fn get_chat_id(&self) -> &String;
    fn get_cloned_chat_id(&self) -> String;
    fn try_lock_sync(&self) -> Result<MutexGuard<'_, bool>, VoiceflowError>;
    fn last_interaction(&self) -> Arc<RwLock<Option<i64>>>;
    fn status(&self) -> Arc<RwLock<bool>>;
    async fn get_last_interaction(&self) -> Option<i64> {
        let binding = self.last_interaction();
        let last_interaction = binding.read().await;
        *last_interaction
    }
    async fn set_last_interaction(&self, interaction: i64) -> (){
        let binding = self.last_interaction();
        let mut last_interaction = binding.write().await;
        *last_interaction = Some(interaction);
    }
    async fn is_valid(&self, valid_duration: &Option<i64>) -> bool{
        if let Some(duration) = valid_duration{
            let now = Utc::now().timestamp();
            if let Some(last_interaction) = &self.get_last_interaction().await{
                !(now - last_interaction > *duration)
            }
            else{
                false
            }
        }
        else{
            true
        }
    }

    async fn activate(&self) ->  (){
        let binding = self.status();
        let mut write_status = binding.write().await;
        *write_status = true;
    }
    async fn deactivate(&self) ->  (){
        let binding = self.status();
        let mut write_status = binding.write().await;
        *write_status = false;
    }
    async fn is_active(&self) -> bool{
        let binding = self.status();
        let status = binding.read().await;
        *status
    }
}