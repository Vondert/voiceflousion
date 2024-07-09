use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use chrono::Utc;
use tokio::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::integrations::utils::traits::Session;
use crate::voiceflow::{VoiceflousionError, VoiceflowBlock, VoiceflowMessage};

pub struct SessionWrapper<S: Session>{
    session: S,
    previous_message: Arc<RwLock<Option<VoiceflowBlock>>>,
    lock: Arc<Mutex<bool>>,
}

impl<S: Session> Deref for SessionWrapper<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl<S: Session> DerefMut for SessionWrapper<S>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.session
    }
}

impl<S: Session> SessionWrapper<S>{
    pub fn new(session: S) -> Self{
        Self{
            session,
            previous_message: Arc::new(RwLock::new(None)),
            lock: Arc::new(Mutex::new(true)),
        }
    }

    pub async fn previous_message(&self) -> RwLockWriteGuard<'_, Option<VoiceflowBlock>> {
        let binding = &self.previous_message;
        let message = binding.write().await;
        message
    }
    pub async fn set_previous_message(&self, mut message: VoiceflowMessage) -> (){
        let binding = &self.previous_message;
        let mut previous = binding.write().await;
        *previous = message.pop();
    }
    pub fn try_lock_sync(&self) -> Result<MutexGuard<'_, bool>, VoiceflousionError>{
        let binding = &self.lock;
        binding.try_lock().map_err(|_| VoiceflousionError::SessionLockError)
    }
    pub async fn get_last_interaction(&self) -> Option<i64> {
        let binding = self.last_interaction();
        let last_interaction = binding.read().await;
        *last_interaction
    }
    pub async fn set_last_interaction(&self, interaction: i64) -> (){
        let binding = self.last_interaction();
        let mut last_interaction = binding.write().await;
        *last_interaction = Some(interaction);
    }
    pub async fn is_valid(&self, valid_duration: &Option<i64>) -> bool{
        if let Some(duration) = valid_duration{
            let now = Utc::now().timestamp();
            if let Some(last_interaction) = self.get_last_interaction().await{
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
    pub async fn activate(&self) ->  (){
        let binding = self.status();
        let mut write_status = binding.write().await;
        *write_status = true;
    }
    pub async fn deactivate(&self) ->  (){
        let binding = self.status();
        let mut write_status = binding.write().await;
        *write_status = false;
    }
    pub async fn is_active(&self) -> bool{
        let binding = self.status();
        let status = binding.read().await;
        *status
    }
}