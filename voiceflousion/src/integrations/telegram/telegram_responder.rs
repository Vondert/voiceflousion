use std::ops::Deref;
use async_trait::async_trait;
use reqwest::Response;
use serde::Deserialize;
use crate::core::base_structs::ResponderBase;
use crate::core::traits::Responder;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

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
    /// The base structure that provides core functionalities.
    responder_base: ResponderBase
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

impl Deref for TelegramResponder {
    type Target = ResponderBase;

    fn deref(&self) -> &Self::Target {
        &self.responder_base
    }
}

#[async_trait]
impl Responder for TelegramResponder {

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
    /// A `VoiceflousionResult` containing the `TelegramResponder` instance or a `VoiceflousionError` if the process fails.
    async fn from_response(response: Response, content: VoiceflowBlock) -> VoiceflousionResult<Self> {
        let body = response.json::<ResponseBody>().await
            .map_err(|e| VoiceflousionError::ClientResponseReadingError("TelegramResponder".to_string(), e.to_string()))?;

        let result = body.result;

        Ok(Self {
            bot_id: result.from.id.to_string(),
            chat_id: result.chat.id.to_string(),
            responder_base: ResponderBase::new(result.message_id.to_string(), content, result.date)
        })
    }
}