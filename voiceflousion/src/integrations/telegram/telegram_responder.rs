use async_trait::async_trait;
use reqwest::Response;
use serde::Deserialize;
use crate::core::traits::Responder;
use crate::core::voiceflow::{VoiceflousionError, VoiceflowBlock};

/// Represents a responder for Telegram interactions.
///
/// `TelegramResponder` contains details of the message sent to Telegram, including
/// the bot ID, chat ID, message ID, date, and the content of the message.
#[derive(Debug)]
pub struct TelegramResponder {
    /// The ID of the bot that sent the message.
    bot_id: String,
    /// The ID of the chat where the message was sent.
    chat_id: String,
    /// The ID of the message.
    message_id: String,
    /// The date when the message was sent.
    date: i64,
    /// The content of the message.
    message_content: VoiceflowBlock,
}

/// Represents the chat details in a Telegram response.
#[derive(Debug, Deserialize)]
struct Chat {
    /// The ID of the chat.
    id: u64,
}

/// Represents the bot details in a Telegram response.
#[derive(Debug, Deserialize)]
struct From {
    /// The ID of the bot.
    id: u64,
}

/// Represents the result details in a Telegram response.
#[derive(Debug, Deserialize)]
struct TelegramResult {
    /// The chat details.
    chat: Chat,
    /// The date when the message was sent.
    date: i64,
    /// The bot details.
    from: From,
    /// The ID of the message.
    message_id: u64,
}

/// Represents the body of a Telegram response.
#[derive(Debug, Deserialize)]
struct ResponseBody {
    /// The result details.
    result: TelegramResult,
}

impl TelegramResponder{

    /// Returns a reference to the bot ID.
    ///
    /// # Returns
    ///
    /// A reference to the bot ID string.
    pub fn bot_id(&self) -> &String{
        &self.bot_id
    }

    /// Returns a reference to the chat ID.
    ///
    /// # Returns
    ///
    /// A reference to the chat ID string.
    pub fn chat_id(&self) -> &String{
        &self.chat_id
    }
}

#[async_trait]
impl Responder for TelegramResponder {
    /// Returns a reference to the message ID.
    ///
    /// # Returns
    ///
    /// A reference to the message ID string.
    fn message_id(&self) -> &String {
        &self.message_id
    }

    /// Returns a reference to the content of the message.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowBlock` representing the message content.
    fn message_content(&self) -> &VoiceflowBlock {
        &self.message_content
    }

    /// Returns the date of the message.
    ///
    /// # Returns
    ///
    /// The date of the message as an `i64` timestamp.
    fn date(&self) -> i64 {
        self.date
    }

    /// Creates an instance of the `TelegramResponder` from HTTP response.
    ///
    /// This method processes the HTTP response to extract relevant details and
    /// create an instance of the `TelegramResponder`.
    ///
    /// # Parameters
    ///
    /// * `response` - The HTTP response to process.
    /// * `content` - The content of the message as a `VoiceflowBlock`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `TelegramResponder` instance or a `VoiceflousionError` if the process fails.
    async fn from_response(response: Response, content: VoiceflowBlock) -> Result<Self, VoiceflousionError> {
        let body = response.json::<ResponseBody>().await
            .map_err(|e| VoiceflousionError::ClientResponseReadingError("TelegramResponder".to_string(), e.to_string()))?;

        let result = body.result;

        Ok(Self {
            bot_id: result.from.id.to_string(),
            chat_id: result.chat.id.to_string(),
            message_id: result.message_id.to_string(),
            date: result.date,
            message_content: content,
        })
    }
}