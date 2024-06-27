use serde::Serialize;
use serde_json::Value;
use crate::voiceflow::request_structures::ActionType;
use crate::voiceflow::request_structures::payload::Payload;

#[derive(Debug, Serialize)]
pub struct Action {
    #[serde(rename = "type")]
    action_type: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Payload>,
}

pub struct ActionBuilder {
    action_type: ActionType,
    payload: Option<Payload>,
}

impl ActionBuilder {
    pub fn new(action_type: ActionType) -> Self {
        Self {
            action_type,
            payload: None,
        }
    }

    pub fn text(mut self, text: String) -> Self {
        let json_value: Value = Value::String(text);
        self.payload = Some(Payload::Single(json_value));
        self
    }

    pub fn build(self) -> Action {
        Action {
            action_type: self.action_type,
            payload: self.payload,
        }
    }
}