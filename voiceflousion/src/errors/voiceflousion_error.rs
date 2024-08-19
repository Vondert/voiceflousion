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
    /// Contains a string description and the JSON value that failed to convert.
    VoiceflowBlockConvertationError(String, Value),

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
    DeprecatedError(String, String),

    /// Error occurred due to invalid update.
    ///
    /// Contains the update and error message.
    ValidationError(String, String)
}

/// Type alias for `Result` with a `VoiceflousionError` error type.
///
/// `VoiceflousionResult` is a convenient shorthand for `Result` that defaults to using
/// `VoiceflousionError` for the error type. This can be used throughout the Voiceflousion
/// crate to simplify function signatures and error handling.
pub type VoiceflousionResult<T> = Result<T, VoiceflousionError>;

impl Display for VoiceflousionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        match self {
            VoiceflousionError::VoiceflowBlockConvertationError(error, block) => {
                write!(f, "Failed to convert voiceflow block: {}\n{:?}", error, block)
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
            },
            VoiceflousionError::ValidationError(validated, error) =>{
                write!(f, "Validation failure of {}. {}", validated, error)
            }
        }
    }
}

impl Error for VoiceflousionError {}