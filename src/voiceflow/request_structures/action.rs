use serde::Serialize;
use serde_json::Value;
use crate::voiceflow::request_structures::ActionType;
use crate::voiceflow::request_structures::payload::Payload;

#[derive(Debug, Serialize)]
pub(crate) struct Action {
    #[serde(rename = "type")]
    action_type: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Payload>,
}

pub(crate) struct ActionBuilder {
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
    pub fn intent(mut self, name: String, path: String) -> Self {
        let json_value: Value = serde_json::json!({
            "query": name,
            "intent":{
                "name": path
            }
        });
        println!("{:?}", &json_value);
        self.payload = Some(Payload::Object(json_value));
        self
    }
    pub fn path(mut self, button_name: String) -> Self{
        let json_value: Value = serde_json::json!({
            "label": button_name,
        });
        println!("{:?}", &json_value);
        self.payload = Some(Payload::Object(json_value));
        self
    }
    pub fn build(self) -> Action {
        Action {
            action_type: self.action_type,
            payload: self.payload,
        }
    }
}