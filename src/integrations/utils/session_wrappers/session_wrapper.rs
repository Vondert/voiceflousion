use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::integrations::utils::traits::Session;
use crate::voiceflow::{VoiceflousionError, VoiceflowBlock};

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

impl<S: Session> SessionWrapper<S>{
    pub fn new(session: S) -> Self{
        Self{
            session,
            previous_message: Arc::new(RwLock::new(None)),
            lock: Arc::new(Mutex::new(true)),
        }
    }
    pub fn try_lock_sync(&self) -> Result<MutexGuard<'_, bool>, VoiceflousionError>{
        let binding = &self.lock;
        binding.try_lock().map_err(|_| VoiceflousionError::SessionLockError)
    }
    pub async fn previous_message(&self) -> RwLockReadGuard<'_, Option<VoiceflowBlock>> {
        let binding = &self.previous_message;
        let message = binding.read().await;
        message
    }
    pub async fn get_last_interaction(&self) -> Option<i64> {
        let binding = self.last_interaction();
        let last_interaction = binding.read().await;
        *last_interaction
    }
    pub(super) async fn write_previous_message(&self) -> RwLockWriteGuard<'_, Option<VoiceflowBlock>>{
        let binding = &self.previous_message;
        let previous = binding.write().await;
        previous
    }
    pub(super) async fn write_last_interaction(&self) ->  RwLockWriteGuard<'_, Option<i64>>{
        let binding = self.last_interaction();
        let last_interaction = binding.write().await;
        last_interaction
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