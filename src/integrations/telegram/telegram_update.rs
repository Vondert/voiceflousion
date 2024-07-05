use crate::integrations::utils::InteractionType;
use crate::integrations::utils::traits::Update;

pub struct TelegramUpdate{
    bot_id: String,
    chat_id: String,
    interaction_time: i64,
    interaction_type: InteractionType
}
impl TelegramUpdate{
    pub fn new (bot_id: String, chat_id: String, interaction_time: i64,  interaction_type: InteractionType) -> Self{
        Self{
            bot_id,
            chat_id,
            interaction_time,
            interaction_type
        }
    }
}
impl Update for TelegramUpdate{
    fn bot_id(&self) -> &str {
        &self.bot_id
    }

    fn chat_id_cloned(&self) -> String {
        self.chat_id.clone()
    }

    fn interaction_time(&self) -> i64 {
        self.interaction_time
    }

    fn interaction_type(self) -> InteractionType {
        self.interaction_type
    }
}