
#[derive(Debug)]
pub enum ActionType{
    Launch,
    Text,
    Intent
}

impl ToString for ActionType{
    fn to_string(&self) -> String {
        match &self {
            ActionType::Launch => "launch".to_string(),
            ActionType::Text => "text".to_string(),
            ActionType::Intent => "intent".to_string()
        }
    }
}