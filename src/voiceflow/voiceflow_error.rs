use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde_json::Value;

#[derive(Debug)]
pub enum VoiceflowError {
    BlockConvertationError((String, Value)),
    RequestError(String),
    ResponseReadingError(String),
    SessionLockError
}

impl Display for VoiceflowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        match self {
            VoiceflowError::BlockConvertationError(block) => {
                write!(f, "Failed to convert block: {:?}", block)
            },
            VoiceflowError::RequestError(error) => {
                write!(f, "Request to voiceflow failed: {:?}", error)
            },
            VoiceflowError::ResponseReadingError(error) => {
                write!(f, "Voiceflow response reading: {:?}", error)
            },
            VoiceflowError::SessionLockError =>{
                write!(f, "Session is locked")
            }
        }
    }
}

impl Error for VoiceflowError {}