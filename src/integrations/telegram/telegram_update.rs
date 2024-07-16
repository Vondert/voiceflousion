use serde_json::Value;
use crate::integrations::utils::InteractionType;
use crate::integrations::utils::traits::Update;
use crate::voiceflow::VoiceflousionError;

#[derive(Debug)]
pub struct TelegramUpdate{
    chat_id: String,
    message_id: String,
    interaction_time: i64,
    interaction_type: InteractionType,
    update_id: String,
    carousel_card_index: Option<usize>,
}

impl TelegramUpdate{
    pub fn new (chat_id: String, message_id: String, interaction_time: i64,  interaction_type: InteractionType, update_id: String, carousel_card_index: Option<usize>) -> Self{
        Self{
            chat_id,
            message_id,
            interaction_time,
            interaction_type,
            update_id,
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

    fn chat_id(&self) -> &String {
        &self.chat_id
    }

    fn message_id(&self) -> &String {
        &self.message_id
    }

    fn interaction_time(&self) -> i64 {
        self.interaction_time
    }

    fn interaction_type(&self) -> &InteractionType {
        &self.interaction_type
    }

    fn from_request_body(body: Value) -> Result<Self, VoiceflousionError> {
        let is_message = body.get("message").is_some();
        let update_data = (if is_message {
            body.get("message")
        }
        else {
            body.get("callback_query").and_then(|q| q.get("message"))
        })
        .ok_or_else(|| VoiceflousionError::RequestError("Missing message data".into()))?;

        let chat_id = update_data.get("chat")
            .and_then(|chat| chat.get("id"))
            .and_then(|id| id.as_i64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::RequestError("Missing chat id".into()))?;

        let message_id = update_data.get("message_id")
            .and_then(|id| id.as_i64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::RequestError("Missing message id".into()))?;

        let text = if is_message {
            update_data.get("text").and_then(|t| t.as_str()).unwrap_or_default()
        } else {
            update_data.get("caption").and_then(|c| c.as_str()).unwrap_or_default()
        }.to_string();

        let interaction_time = update_data.get("date")
            .and_then(|date| date.as_i64())
            .ok_or_else(|| VoiceflousionError::RequestError("Missing date".into()))?;

        let update_id = body.get("update_id")
            .and_then(|id| id.as_i64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::RequestError("Missing update id".into()))?;

        let callback_data = if !is_message {
            Some(body.get("callback_query")
                .and_then(|q| q.get("data"))
                .and_then(|data| data.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| VoiceflousionError::RequestError("Missing update id".into()))?)
        }
        else{
            None
        };

        let carousel_card_index = callback_data.as_ref()
            .and_then(|data| data.strip_prefix("c_").and_then(|index| index.parse::<usize>().ok()));

        let interaction_type = InteractionType::new(text, callback_data);

        Ok(TelegramUpdate {
            chat_id,
            message_id,
            interaction_time,
            interaction_type,
            update_id,
            carousel_card_index,
        })
    }
}