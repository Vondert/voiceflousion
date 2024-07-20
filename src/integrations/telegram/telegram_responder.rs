use async_trait::async_trait;
use reqwest::Response;
use serde::Deserialize;
use crate::integrations::utils::traits::Responder;
use crate::voiceflow::{VoiceflousionError, VoiceflowBlock};

#[derive(Debug)]
pub struct TelegramResponder {
    bot_id: String,
    chat_id: String,
    message_id: String,
    date: i64,
    message_content: VoiceflowBlock
}

#[derive(Debug, Deserialize)]
struct Chat {
    id: u64,
}

#[derive(Debug, Deserialize)]
struct From {
    id: u64,
}

#[derive(Debug, Deserialize)]
struct TelegramResult {
    chat: Chat,
    date: i64,
    from: From,
    message_id: u64,
}

#[derive(Debug, Deserialize)]
struct ResponseBody {
    result: TelegramResult,
}
#[async_trait]
impl Responder for TelegramResponder {
    fn message_id(&self) -> &String {
        &self.message_id
    }

    fn message_content(&self) -> &VoiceflowBlock {
        &self.message_content
    }

    fn date(&self) -> i64 {
        self.date
    }

    async fn from_response(response: Response, content: VoiceflowBlock) -> Result<Self, VoiceflousionError> {
        let body = response.json::<ResponseBody>().await.map_err(|e| VoiceflousionError::ClientResponseReadingError("TelegramResponder".to_string(), e.to_string()))?;

        let result = body.result;

        Ok(
            Self {
                bot_id: result.from.id.to_string(),
                chat_id: result.chat.id.to_string(),
                message_id: result.message_id.to_string(),
                date: result.date,
                message_content: content
            }
        )
    }
}