use crate::integrations::interaction_type::InteractionType;

pub trait Update{
    fn bot_id(&self) -> &str;
    fn chat_id_cloned(&self) -> String;
    fn interaction_time(&self) -> i64;
    fn interaction_type(self) -> InteractionType;
}