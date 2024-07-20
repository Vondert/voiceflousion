use serde_json::Value;
use crate::integrations::utils::subtypes::InteractionType;
use crate::integrations::utils::traits::Update;
use crate::voiceflow::VoiceflousionError;

/// Represents an update received from Telegram.
///
/// `TelegramUpdate` holds the details of an update from Telegram, such as chat ID,
/// interaction time, interaction type, update ID, message ID, and optionally the carousel card index.
#[derive(Debug)]
pub struct TelegramUpdate {
    /// The chat ID associated with the update.
    chat_id: String,
    /// The interaction time of the update.
    interaction_time: i64,
    /// The type of interaction.
    interaction_type: InteractionType,
    /// The update ID.
    update_id: String,
    /// The message ID.
    message_id: String,
    /// The optional carousel card index.
    carousel_card_index: Option<usize>,
}

impl TelegramUpdate {
    /// Creates a new `TelegramUpdate`.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID associated with the update.
    /// * `message_id` - The message ID associated with the update.
    /// * `interaction_time` - The interaction time of the update.
    /// * `interaction_type` - The type of interaction.
    /// * `update_id` - The update ID.
    /// * `carousel_card_index` - The optional carousel card index.
    ///
    /// # Returns
    ///
    /// A new instance of `TelegramUpdate`.
    ///
    /// # Example
    ///
    /// ```
    /// let update = TelegramUpdate::new("chat_id".to_string(), "message_id".to_string(), 1627554661, interaction_type, "update_id".to_string(), Some(0));
    /// ```
    pub fn new(chat_id: String, message_id: String, interaction_time: i64, interaction_type: InteractionType, update_id: String, carousel_card_index: Option<usize>) -> Self {
        Self {
            chat_id,
            message_id,
            interaction_time,
            interaction_type,
            update_id,
            carousel_card_index,
        }
    }

    /// Returns the carousel card index.
    ///
    /// # Returns
    ///
    /// An optional `usize` representing the carousel card index.
    ///
    /// # Example
    ///
    /// ```
    /// let index = update.carousel_card_index();
    /// ```
    pub fn carousel_card_index(&self) -> Option<usize> {
        self.carousel_card_index
    }

    /// Returns the message ID.
    ///
    /// # Returns
    ///
    /// A reference to the message ID string.
    ///
    /// # Example
    ///
    /// ```
    /// let message_id = update.message_id();
    /// ```
    pub fn message_id(&self) -> &String {
        &self.message_id
    }
}

impl Update for TelegramUpdate {
    /// Returns the chat ID associated with the update.
    ///
    /// # Returns
    ///
    /// A reference to the chat ID string.
    ///
    /// # Example
    ///
    /// ```
    /// let chat_id = update.chat_id();
    /// ```
    fn chat_id(&self) -> &String {
        &self.chat_id
    }

    /// Returns the update ID.
    ///
    /// # Returns
    ///
    /// A reference to the update ID string.
    ///
    /// # Example
    ///
    /// ```
    /// let update_id = update.update_id();
    /// ```
    fn update_id(&self) -> &String {
        &self.update_id
    }

    /// Returns the interaction time.
    ///
    /// # Returns
    ///
    /// An `i64` representing the interaction time.
    ///
    /// # Example
    ///
    /// ```
    /// let interaction_time = update.interaction_time();
    /// ```
    fn interaction_time(&self) -> i64 {
        self.interaction_time
    }

    /// Returns the type of interaction.
    ///
    /// # Returns
    ///
    /// A reference to the `InteractionType`.
    ///
    /// # Example
    ///
    /// ```
    /// let interaction_type = update.interaction_type();
    /// ```
    fn interaction_type(&self) -> &InteractionType {
        &self.interaction_type
    }

    /// Creates an update from a JSON request body.
    ///
    /// # Parameters
    ///
    /// * `body` - A JSON `Value` representing the request body.
    ///
    /// # Returns
    ///
    /// A `Result` containing the update or a `VoiceflousionError` if the conversion fails.
    ///
    /// # Example
    ///
    /// ```
    /// let update = TelegramUpdate::from_request_body(body)?;
    /// ```
    fn from_request_body(body: Value) -> Result<Self, VoiceflousionError> {
        let is_message = body.get("message").is_some();
        let update_data = (if is_message {
            body.get("message")
        } else {
            body.get("callback_query").and_then(|q| q.get("message"))
        })
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate message".to_string(), body.clone()))?;

        let chat_id = update_data.get("chat")
            .and_then(|chat| chat.get("id"))
            .and_then(|id| id.as_i64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate chat id".to_string(), update_data.clone()))?;

        let message_id = update_data.get("message_id")
            .and_then(|id| id.as_i64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate message id".to_string(), update_data.clone()))?;

        let text = if is_message {
            update_data.get("text").and_then(|t| t.as_str()).unwrap_or_default()
        } else {
            update_data.get("caption").and_then(|c| c.as_str()).unwrap_or_default()
        }.to_string();

        let interaction_time = update_data.get("date")
            .and_then(|date| date.as_i64())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate interaction timestamp".to_string(), update_data.clone()))?;

        let update_id = body.get("update_id")
            .and_then(|id| id.as_i64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate update id".to_string(), body.clone()))?;

        let callback_data = if !is_message {
            Some(body.get("callback_query")
                .and_then(|q| q.get("data"))
                .and_then(|data| data.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate callback data".to_string(), body.clone()))?)
        } else {
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
