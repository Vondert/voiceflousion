use crate::integrations::utils::interaction_type::InteractionType;

pub trait Update: Send + Sync{
    fn bot_id(&self) -> &String;
    fn chat_id(&self) -> &String;
    fn interaction_time(&self) -> i64;
    fn interaction_type(&self) -> &InteractionType;
}