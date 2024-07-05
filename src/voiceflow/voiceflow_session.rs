use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
pub struct VoiceflowSession {
    #[serde(rename = "sessionID")]
    session_id: String,
    #[serde(rename = "userID")]
    user_id: String
}

impl VoiceflowSession {
    fn new(session_id: String, user_id: String) -> Self{
        Self{
          session_id,
          user_id
        }
    }
    pub fn from_chat_id(chat_id: &str) -> Self {
        fn hash(input: &str, suffix: &str) -> String {
            let mut hasher = Sha256::new();
            hasher.update(input);
            hasher.update(suffix);
            format!("{:x}", hasher.finalize())
        }

        let session_id = hash(chat_id, "session_id");
        let user_id = hash(chat_id, "user_id");
        Self::new(session_id, user_id)
    }
}