use std::fmt::Display;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ActionType {
    #[serde(rename = "launch")]
    Launch,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "intent")]
    Intent,
}

impl Display for ActionType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            ActionType::Launch => "launch".to_string(),
            ActionType::Text => "text".to_string(),
            ActionType::Intent => "intent".to_string()
        };
        write!(f, "{}", str)
    }
}