use serde::Serialize;
use serde_json::Value;
use crate::core::voiceflow::request_structures::ActionType;
use crate::core::voiceflow::request_structures::payload::Payload;

/// Represents an action to be performed in the Voiceflow API.
///
/// `Action` contains the type of action and an optional payload associated with the action.
#[derive(Debug, Serialize)]
pub(crate) struct Action {
    /// The type of action to be performed.
    #[serde(rename = "type")]
    action_type: ActionType,

    /// The optional payload associated with the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Payload>,
}

/// A builder for creating an `Action`.
///
/// `ActionBuilder` allows for the incremental construction of an `Action` by setting various fields.
pub(crate) struct ActionBuilder {
    action_type: ActionType,
    payload: Option<Payload>,
}

impl ActionBuilder {
    /// Creates a new `ActionBuilder` with the specified action type.
    ///
    /// # Parameters
    ///
    /// * `action_type` - The type of action to be performed.
    ///
    /// # Returns
    ///
    /// A new instance of `ActionBuilder`.
    pub fn new(action_type: ActionType) -> Self {
        Self {
            action_type,
            payload: None,
        }
    }

    /// Sets the text payload for the `Action`.
    ///
    /// # Parameters
    ///
    /// * `text` - The text to be included in the payload.
    ///
    /// # Returns
    ///
    /// The `ActionBuilder` with the text payload set.
    pub fn text(mut self, text: String) -> Self {
        let json_value: Value = Value::String(text);
        self.payload = Some(Payload::Single(json_value));
        self
    }

    /// Sets the intent payload for the `Action`.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the intent.
    /// * `path` - The path associated with the intent.
    ///
    /// # Returns
    ///
    /// The `ActionBuilder` with the intent payload set.
    pub fn intent(mut self, name: String, path: String) -> Self {
        let json_value: Value = serde_json::json!({
            "query": name,
            "intent":{
                "name": path
            }
        });
        self.payload = Some(Payload::Object(json_value));
        self
    }
    /// Sets the path payload for the `Action`.
    ///
    /// # Parameters
    ///
    /// * `text` - The text to be included in the path payload.
    ///
    /// # Returns
    ///
    /// The `ActionBuilder` with the path payload set.
    pub fn path(mut self, text: String) -> Self{
        let json_value: Value = serde_json::json!({
            "label": text,
            "index": 0,
        });
        self.payload = Some(Payload::Object(json_value));
        self
    }

    /// Builds the `Action`.
    ///
    /// # Returns
    ///
    /// A new instance of `Action` with the configured fields.
    pub fn build(self) -> Action {
        Action {
            action_type: self.action_type,
            payload: self.payload,
        }
    }
}