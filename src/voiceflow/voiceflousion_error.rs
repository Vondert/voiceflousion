use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde_json::Value;

#[derive(Debug)]
pub enum VoiceflousionError {
    BlockConvertationError((String, Value)),
    RequestError(String),
    ResponseReadingError(String),
    SessionLockError
}

impl Display for VoiceflousionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        match self {
            VoiceflousionError::BlockConvertationError(block) => {
                write!(f, "Failed to convert block: {:?}", block)
            },
            VoiceflousionError::RequestError(error) => {
                write!(f, "Request to voiceflow failed: {:?}", error)
            },
            VoiceflousionError::ResponseReadingError(error) => {
                write!(f, "Voiceflow response reading: {:?}", error)
            },
            VoiceflousionError::SessionLockError =>{
                write!(f, "Session is locked")
            }
        }
    }
}

impl Error for VoiceflousionError {}