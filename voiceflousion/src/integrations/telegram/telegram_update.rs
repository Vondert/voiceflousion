use std::ops::Deref;
use serde_json::Value;
use crate::core::base_structs::UpdateBase;
use crate::core::subtypes::InteractionType;
use crate::core::traits::Update;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::utils::ButtonCallbackData;

/// Represents an update received from Telegram.
///
/// `TelegramUpdate` holds the details of an update from Telegram, such as update base, message ID, and optionally the carousel card index.
#[derive(Debug)]
pub struct TelegramUpdate {
    /// The base structure that provides core functionalities.
    update_base: UpdateBase,
    /// The message ID.
    message_id: String,
    /// The optional carousel switch direction.
    carousel_direction: Option<bool>,
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
    /// let button_index = 0usize;
    /// let interaction_type = InteractionType::new("message".to_string(), Some(button_index), None);
    /// let update = TelegramUpdate::new("chat_id".to_string(), "message_id".to_string(), 1627554661, interaction_type, "update_id".to_string(), Some(true));
    /// ```
    pub fn new(chat_id: String, message_id: String, interaction_time: i64, interaction_type: InteractionType, update_id: String, carousel_direction: Option<bool>) -> Self {
        Self {
            update_base: UpdateBase::new(chat_id, interaction_time, interaction_type, update_id),
            message_id,
            carousel_direction,
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
    /// let button_index = 0usize;
    /// let interaction_type = InteractionType::new("message".to_string(), Some(button_index), None);
    /// let update = TelegramUpdate::new("chat_id".to_string(), "message_id".to_string(), 1627554661, interaction_type, "update_id".to_string(), Some(true));
    /// let index = update.carousel_card_index();
    /// ```
    pub fn carousel_card_index(&self) -> Option<bool> {
        self.carousel_direction
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
    /// let button_index = 0usize;
    /// let interaction_type = InteractionType::new("message".to_string(), Some(button_index), None);
    /// let update = TelegramUpdate::new("chat_id".to_string(), "message_id".to_string(), 1627554661, interaction_type, "update_id".to_string(), Some(true));
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
        let chat_id = update_data["chat"].get("id")
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

        let mut carousel_direction = None;
        let mut button_index = None;

        // Extract the carousel card index and direction if button interaction
        if !is_message {
            let data = body["callback_query"].get("data")
                .and_then(|data| data.as_str())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate callback data".to_string(), body.clone()))?;

            let callback_data: ButtonCallbackData = serde_json::from_str(data)
                .map_err(|_error| VoiceflousionError::ClientUpdateConvertationError("TelegramUpdate callback data must be a valid JSON string".to_string(), body.clone()))?;

            carousel_direction = callback_data.direction();
            button_index = callback_data.index();
        }

        // Create an InteractionType from the text, path and button index
        let interaction_type = InteractionType::new(text, button_index, carousel_direction);

        // Return the constructed TelegramUpdate
        Ok(TelegramUpdate::new(
            chat_id,
            message_id,
            interaction_time,
            interaction_type,
            update_id,
            carousel_direction,
        ))
    }
}
