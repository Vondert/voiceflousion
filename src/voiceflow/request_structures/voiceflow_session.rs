use serde::Serialize;
use sha2::{Digest, Sha256};
use tokio::sync::{Mutex, MutexGuard, TryLockError};
use crate::voiceflow::VoiceflowError;

#[derive(Debug, Serialize)]
pub struct VoiceflowSession {
    #[serde(rename = "sessionID")]
    session_id: String,
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(skip)]
    lock: Mutex<bool>
}

impl VoiceflowSession {
    fn new(session_id: String, user_id: String) -> Self{
        Self{
          session_id,
          user_id,
          lock: Mutex::new(true)
        }
    }
    pub fn from_chat_id(chat_id: &str) -> Self{
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, &chat_id);
        Digest::update(&mut hasher, "session_id");
        let session_id = format!("{:x}", hasher.finalize());

        hasher = Sha256::new();
        Digest::update(&mut hasher, &chat_id);
        Digest::update(&mut hasher, "user_id");
        let user_id = format!("{:x}", hasher.finalize());
        Self::new(session_id, user_id)
    }
    pub fn try_lock(&self) -> Result<MutexGuard<'_, bool>, VoiceflowError> {
        self.lock.try_lock().map_err(|_| VoiceflowError::SessionLockError)
    }
}