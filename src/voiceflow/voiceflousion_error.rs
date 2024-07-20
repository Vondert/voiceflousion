use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde_json::Value;

#[derive(Debug)]
pub enum VoiceflousionError {
    VoiceflowBlockConvertationError((String, Value)),
    VoiceflowRequestError(String, String, String),
    VoiceflowResponseReadingError(String),
    ClientUpdateConvertationError(String, Value),
    ClientRequestError(String, String),
    ClientRequestInvalidBodyError(String, String),
    ClientResponseReadingError(String, String),
    SessionLockError(String),
    DeprecatedError(String, String)
}

impl Display for VoiceflousionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        match self {
            VoiceflousionError::VoiceflowBlockConvertationError(block) => {
                write!(f, "Failed to convert voiceflow block: {}\n{:?}", block.0, block.1)
            },
            VoiceflousionError::VoiceflowResponseReadingError(error) => {
                write!(f, "Voiceflow response reading: {}", error)
            },
            VoiceflousionError::VoiceflowRequestError(project_id, version_id, error) =>{
                write!(f, "Request to voiceflow failed: \n-project_id: {}\n-version_id: {}\n{}", project_id, version_id, error)
            }
            VoiceflousionError::ClientUpdateConvertationError(place, value) => {
                write!(f, "Failed to convert client update: {}\n{:?}", place, value)
            },
            VoiceflousionError::ClientRequestError(message, error) => {
                write!(f, "Client request failed: {}\n{}", message, error)
            },
            VoiceflousionError::ClientRequestInvalidBodyError(place, error) => {
                write!(f, "Client request invalid body: {}\n{}", place, error)
            },
            VoiceflousionError::ClientResponseReadingError(place, error) => {
                write!(f, "Client response reading: {}\n{}", place, error)
            },
            VoiceflousionError::SessionLockError(chat_id) =>{
                write!(f, "Session {} is locked", chat_id)
            }
            VoiceflousionError::DeprecatedError(chat_id, update_id) =>{
                write!(f, "Update {} from chat {} is deprecated", update_id, chat_id)
            }
        }
    }
}

impl Error for VoiceflousionError {}