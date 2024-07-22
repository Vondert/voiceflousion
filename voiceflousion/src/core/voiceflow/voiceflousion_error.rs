use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde_json::Value;

/// Represents various errors that can occur in the Voiceflousion crate.
///
/// `VoiceflousionError` enumerates different types of errors that might occur,
/// providing detailed context for each error type.
#[derive(Debug)]
pub enum VoiceflousionError {
    /// Error occurred while converting a Voiceflow block.
    ///
    /// Contains a tuple with a string description and the JSON value that failed to convert.
    VoiceflowBlockConvertationError((String, Value)),

    /// Error occurred while making a request to Voiceflow.
    ///
    /// Contains the project ID, version ID, and a string description of the error.
    VoiceflowRequestError(String, String, String),

    /// Error occurred while reading a response from Voiceflow.
    ///
    /// Contains a string description of the error.
    VoiceflowResponseReadingError(String),

    /// Error occurred while converting a client update.
    ///
    /// Contains a string description and the JSON value that failed to convert.
    ClientUpdateConvertationError(String, Value),

    /// Error occurred during a client request.
    ///
    /// Contains a string description and the error message.
    ClientRequestError(String, String),

    /// Error occurred due to an invalid body in a client request.
    ///
    /// Contains a string description and the error message.
    ClientRequestInvalidBodyError(String, String),

    /// Error occurred while reading a client response.
    ///
    /// Contains a string description and the error message.
    ClientResponseReadingError(String, String),

    /// Error occurred while trying to lock a session.
    ///
    /// Contains the chat ID of the session that is locked.
    SessionLockError(String),

    /// Error occurred due to a deprecated update.
    ///
    /// Contains the chat ID and update ID of the deprecated update.
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