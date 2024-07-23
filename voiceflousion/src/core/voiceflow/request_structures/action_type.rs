use std::fmt::Display;
use serde::{Serialize, Serializer};

/// Represents the type of action to be performed in the Voiceflow API.
#[derive(Debug)]
pub(crate) enum ActionType {
    /// Launch action.
    Launch,

    /// Text action.
    Text,

    /// Intent action.
    Intent,

    /// Path action with an associated path string.
    Path(String),
}
impl Serialize for ActionType {
    /// Serializes the `ActionType` to a string for use in JSON.
    ///
    /// # Parameters
    ///
    /// * `serializer` - The serializer to use.
    ///
    /// # Returns
    ///
    /// A `Result` containing the serialized string or an error.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ActionType::Launch => serializer.serialize_str("launch"),
            ActionType::Text => serializer.serialize_str("text"),
            ActionType::Intent => serializer.serialize_str("intent"),
            ActionType::Path(path) => serializer.serialize_str(path),
        }
    }
}
impl Display for ActionType{
    /// Formats the `ActionType` for display.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            ActionType::Launch => "launch".to_string(),
            ActionType::Text => "text".to_string(),
            ActionType::Intent => "intent".to_string(),
            ActionType::Path(path) => path.clone()
        };
        write!(f, "{}", str)
    }
}