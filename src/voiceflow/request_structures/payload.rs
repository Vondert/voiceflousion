use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(crate) enum Payload {
    Single(Value),
    Object(Value),
}