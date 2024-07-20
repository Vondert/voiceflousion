use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::integrations::utils::subtypes::{AtomicTimestamp, SentMessage};
use crate::voiceflow::VoiceflowSession;

pub struct Session {
    chat_id: String,
    status: Arc<AtomicBool>,
    pub(super) last_interaction: Arc<AtomicTimestamp>,
    previous_message: Arc<RwLock<Option<SentMessage>>>,
    voiceflow_session: VoiceflowSession,
    pub(super) lock: Arc<Mutex<bool>>,
}


impl Session {
    pub fn new(chat_id: String, last_interaction: Option<i64>, status: bool) -> Self{
        let voiceflow_session = VoiceflowSession::from_chat_id(&chat_id);
        Self{
            chat_id,
            voiceflow_session,
            status: Arc::new(AtomicBool::new(status)),
            last_interaction: Arc::new(AtomicTimestamp::new(last_interaction)),
            previous_message: Arc::new(RwLock::new(None)),
            lock: Arc::new(Mutex::new(true)),
        }
    }
    pub async fn previous_message(&self) -> RwLockReadGuard<'_, Option<SentMessage>> {
        let binding = &self.previous_message;
        let message = binding.read().await;
        message
    }
    pub(super) async fn write_previous_message(&self) -> RwLockWriteGuard<'_, Option<SentMessage>>{
        let binding = &self.previous_message;
        let previous = binding.write().await;
        previous
    }
    pub fn get_last_interaction(&self) -> Option<i64> {
        self.last_interaction.load(Ordering::SeqCst)
    }
    pub fn activate(&self) ->  (){
        self.status.store(true, Ordering::Release)
    }
    pub fn deactivate(&self) ->  (){
        self.status.store(false, Ordering::Release)
    }
    pub fn is_active(&self) -> bool{
        self.status.load(Ordering::Acquire)
    }
    pub fn get_chat_id(&self) -> &String {
        &self.chat_id
    }
    pub fn get_cloned_chat_id(&self) -> String {
        self.get_chat_id().clone()
    }
    pub fn voiceflow_session(&self) -> &VoiceflowSession {
        &self.voiceflow_session
    }
}