use crate::integrations::utils::InteractionType;
use crate::integrations::utils::traits::Update;

pub struct TelegramUpdate{
    bot_id: String,
    chat_id: String,
    interaction_time: i64,
    interaction_type: InteractionType,
    carousel_card_index: Option<usize>
}
impl TelegramUpdate{
    pub fn new (bot_id: String, chat_id: String, interaction_time: i64,  interaction_type: InteractionType, carousel_card_index: Option<usize>) -> Self{
        Self{
            bot_id,
            chat_id,
            interaction_time,
            interaction_type,
            carousel_card_index
        }
    }
}
impl TelegramUpdate{
    pub fn carousel_card_index(&self) -> Option<usize>{
        self.carousel_card_index
    }
}
impl Update for TelegramUpdate{
    fn bot_id(&self) -> &String {
        &self.bot_id
    }

    fn chat_id(&self) -> &String {
        &self.chat_id
    }

    fn interaction_time(&self) -> i64 {
        self.interaction_time
    }

    fn interaction_type(&self) -> &InteractionType {
        &self.interaction_type
    }
}