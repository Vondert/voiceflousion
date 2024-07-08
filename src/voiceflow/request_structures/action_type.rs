use std::fmt::Display;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub(crate) enum ActionType {
    Launch,
    Text,
    Intent,
    Path(String)
}
impl Serialize for ActionType {
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