use std::ops::Deref;
use serde_json::Value;
use crate::core::base_structs::UpdateBase;
use crate::core::subtypes::InteractionType;
use crate::core::traits::Update;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents an update received from Telegram.
///
/// `TelegramUpdate` holds the details of an update from Telegram, such as update base, message ID, and optionally the carousel card index.
#[derive(Debug)]
pub struct TelegramUpdate {
    /// The base structure that provides core functionalities.
    update_base: UpdateBase,
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
    /// use voiceflousion::core::subtypes::InteractionType;
    /// use voiceflousion::core::traits::Update;
    /// use voiceflousion::integrations::telegram::TelegramUpdate;
    ///
    /// let interaction_type = InteractionType::new("message".to_string(), Some("path".to_string()), None);
    /// let update = TelegramUpdate::new("chat_id".to_string(), "message_id".to_string(), 1627554661, interaction_type, "update_id".to_string(), Some(0));
    /// ```
    pub fn new(chat_id: String, message_id: String, interaction_time: i64, interaction_type: InteractionType, update_id: String, carousel_card_index: Option<usize>) -> Self {
        Self {
            update_base: UpdateBase::new(chat_id, interaction_time, interaction_type, update_id),
            message_id,
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
    /// use voiceflousion::core::subtypes::InteractionType;
    /// use voiceflousion::core::traits::Update;
    /// use voiceflousion::integrations::telegram::TelegramUpdate;
    ///
    /// let interaction_type = InteractionType::new("message".to_string(), Some("path".to_string()), None);
    /// let update = TelegramUpdate::new("chat_id".to_string(), "message_id".to_string(), 1627554661, interaction_type, "update_id".to_string(), Some(0));
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
    /// use voiceflousion::core::subtypes::InteractionType;
    /// use voiceflousion::core::traits::Update;
    /// use voiceflousion::integrations::telegram::TelegramUpdate;
    ///
    /// let interaction_type = InteractionType::new("message".to_string(), Some("path".to_string()), None);
    /// let update = TelegramUpdate::new("chat_id".to_string(), "message_id".to_string(), 1627554661, interaction_type, "update_id".to_string(), Some(0));
    /// let message_id = update.message_id();
    /// ```
    pub fn message_id(&self) -> &String {
        &self.message_id
    }
}

impl Deref for TelegramUpdate {
    type Target = UpdateBase;

    fn deref(&self) -> &Self::Target {
        &self.update_base
    }
}

impl Update for TelegramUpdate {

    /// Creates an update from a JSON request body.
    ///
    /// # Parameters
    ///
    /// * `body` - A JSON `Value` representing the request body.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing the `TelegramUpdate` or a `VoiceflousionError` if the conversion fails.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use voiceflousion::core::traits::Update;
    /// use voiceflousion::integrations::telegram::TelegramUpdate;
    ///
    /// let body = json!({});
    ///
    /// let update = TelegramUpdate::from_request_body(body);
    /// ```
    fn from_request_body(body: Value) -> VoiceflousionResult<Self> {
        // Check if the update contains a message or a callback query
        let is_message = body.get("message").is_some();
        // Extract the message or callback query data
        let update_data = (if is_message {
            body.get("message")
        } else {
            body.get("callback_query").and_then(|q| q.get("message"))
        })
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate message".to_string(), body.clone()))?;

        // Extract the chat ID from the update data
        let chat_id = update_data.get("chat")
            .and_then(|chat| chat.get("id"))
            .and_then(|id| id.as_i64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate chat id".to_string(), update_data.clone()))?;

        // Extract the message ID from the update data
        let message_id = update_data.get("message_id")
            .and_then(|id| id.as_i64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate message id".to_string(), update_data.clone()))?;


        // Extract the text from the message or caption
        let text = if is_message {
            update_data.get("text").and_then(|t| t.as_str()).unwrap_or_default()
        } else {
            update_data.get("caption").and_then(|c| c.as_str()).unwrap_or_default()
        }.to_string();

        // Extract the interaction time from the update data
        let interaction_time = update_data.get("date")
            .and_then(|date| date.as_i64())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate interaction timestamp".to_string(), update_data.clone()))?;

        // Extract the update ID from the request body
        let update_id = body.get("update_id")
            .and_then(|id| id.as_u64())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate update id".to_string(), body.clone()))?;

        // Extract the callback data if present
        let mut callback_data: Option<Value>  = if !is_message {
            let data = body.get("callback_query")
                .and_then(|q| q.get("data"))
                .and_then(|data| data.as_str())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate callback data".to_string(), body.clone()))?;

            Some(serde_json::from_str(data)
                .map_err(|_error| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate callback data must be a valid JSON string".to_string(), body.clone()))?)

        } else {
            None
        };

        let mut path = None;
        let mut carousel_card_index = None;

        // Extract the carousel card index and path from the callback data if present
        if !is_message {
            let data = callback_data.as_mut().unwrap().as_object_mut();
            if let Some(mut_data) = data{
                carousel_card_index = mut_data.remove("telegram_carousel_card_index")
                    .and_then(|value_index| value_index.as_str().map(|s| s.to_string()))
                    .and_then(|index| index.parse::<usize>().ok());
                path = mut_data.remove("path")
                    .and_then(|value_path| value_path.as_str().map(|s| s.to_string()));
            }
        }

        // Create an InteractionType from the text, path and callback data
        let interaction_type = InteractionType::new(text, path, callback_data);

        // Return the constructed TelegramUpdate
        Ok(TelegramUpdate::new(
            chat_id,
            message_id,
            interaction_time,
            interaction_type,
            update_id,
            carousel_card_index,
        ))
    }
}
