use serde::Serialize;
use serde_json::Value;

/// Represents the payload for an action in the Voiceflow API.
///
/// `Payload` can be either a single JSON value or an object containing multiple JSON values.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(crate) enum Payload {
    /// A single JSON value.
    Single(Value),

    /// An object containing multiple JSON values.
    Object(Value),
}