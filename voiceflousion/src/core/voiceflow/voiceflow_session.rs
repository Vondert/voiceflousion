use serde::Serialize;
use sha2::{Digest, Sha256};

/// Represents a session for the Voiceflow Bot.
///
/// `VoiceflowSession` is used to track the session and user IDs required for interacting
/// with the Voiceflow Bot.
#[derive(Debug, Serialize)]
pub struct VoiceflowSession {
    /// The session ID used in Voiceflow.
    #[serde(rename = "sessionID")]
    session_id: String,

    /// The user ID used in Voiceflow.
    #[serde(rename = "userID")]
    user_id: String,
}

impl VoiceflowSession {
    /// Creates a new Voiceflow session.
    ///
    /// # Parameters
    ///
    /// * `session_id` - The session ID.
    /// * `user_id` - The user ID.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowSession`.
    fn new(session_id: String, user_id: String) -> Self{
        Self{
          session_id,
          user_id
        }
    }
    /// Creates a new Voiceflow session from a chat ID.
    ///
    /// This method generates session and user IDs by hashing the provided chat ID
    /// with specific suffixes to ensure uniqueness.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID used to generate the session and user IDs.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowSession`.
    pub(crate) fn from_chat_id(chat_id: &str) -> Self {
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