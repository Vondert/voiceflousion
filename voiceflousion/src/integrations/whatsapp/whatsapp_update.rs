use std::ops::Deref;
use serde_json::Value;
use crate::core::base_structs::UpdateBase;
use crate::core::subtypes::InteractionType;
use crate::core::traits::Update;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::utils::ButtonCallbackData;

/// Represents an update from a WhatsApp message.
///
/// `WhatsAppUpdate` processes the incoming WhatsApp message and extracts relevant information,
/// such as chat ID, interaction time, interaction type, and update ID.
#[derive(Debug)]
pub struct WhatsAppUpdate {
    /// The base structure that provides core functionalities.
    update_base: UpdateBase,
}

impl Deref for WhatsAppUpdate {
    type Target = UpdateBase;

    fn deref(&self) -> &Self::Target {
        &self.update_base
    }
}


impl WhatsAppUpdate{

    /// Creates a new `WhatsAppUpdate` instance.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The ID of the chat where the interaction occurred.
    /// * `interaction_time` - The timestamp of the interaction.
    /// * `interaction_type` - The type of interaction (e.g., text message, button click).
    /// * `update_id` - The unique ID of the update.
    ///
    /// # Returns
    ///
    /// A new instance of `WhatsAppUpdate`.
    pub fn new(chat_id: String, interaction_time: i64, interaction_type: InteractionType, update_id: String) -> Self {
        Self {
            update_base: UpdateBase::new(chat_id, interaction_time, interaction_type, update_id)
        }
    }
}

impl Update for WhatsAppUpdate{
    /// Constructs a `WhatsAppUpdate` from the request body.
    ///
    /// This method parses the JSON body of an incoming WhatsApp message to extract information
    /// such as the chat ID, interaction time, and the type of interaction (e.g., text, button click).
    ///
    /// # Parameters
    ///
    /// * `body` - The JSON body of the WhatsApp message.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing the `WhatsAppUpdate` instance or an error if the parsing fails.
    ///
    /// # Errors
    ///
    /// This function returns a `VoiceflousionError` if the necessary fields cannot be extracted from the JSON body.
    fn from_request_body(body: Value) -> VoiceflousionResult<Self> {
        // Extract the entry from the body
        let entry = body.get("entry")
            .and_then(|entry_value| entry_value.as_array())
            .and_then(|entry_array| entry_array.first())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate entry".to_string(), body.clone()))?;

        // Extract the value from the entry
        let value = entry["changes"][0].get("value")
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate value".to_string(), entry.clone()))?;

        // Extract the first message from the value
        let message = value.get("messages")
            .and_then(|messages_value| messages_value.as_array())
            .and_then(|messages_array| messages_array.first())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate messages".to_string(), value.clone()))?;

        // Extract chat ID
        let chat_id = message.get("from")
            .and_then(|from| from.as_str())
            .map(|from| from.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate message from (chat id)".to_string(), message.clone()))?;

        // Extract interaction time
        let mut interaction_time = message.get("timestamp")
            .and_then(|date| date.as_str())
            .and_then(|date_str| date_str.parse::<i64>().ok())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate interaction timestamp".to_string(), message.clone()))?;

        // Extract update ID
        let update_id = message.get("id")
            .and_then(|id| id.as_str())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update id".to_string(), message.clone()))?;

        // Determine if the message is an interactive type
        let message_type = message["type"].as_str()
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update type".to_string(), message.clone()))?;
        let is_message = message_type != "interactive";

        // Initialize variables for text, carousel direction, and button index
        let mut text: String = String::new();
        let mut carousel_direction = None;
        let mut button_index = None;

        if is_message {
            // Extract text from the message body
            text = message["text"].get("body")
                .and_then(|body| body.as_str())
                .map(|text_str| text_str.to_string())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update message text".to_string(), message.clone()))?;
        } else {
            // Extract interactive reply data
            let interactive_reply = message["interactive"].get("list_reply")
                .or_else(|| message["interactive"].get("button_reply"))
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update interactive reply".to_string(), message.clone()))?;

            // Extract text from the interactive reply
            text = interactive_reply.get("title")
                .and_then(|text_value| text_value.as_str())
                .map(|text_str| text_str.to_string())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update button title".to_string(), interactive_reply.clone()))?;

            // Extract callback data from the interactive reply
            let data = interactive_reply.get("id")
                .and_then(|data| data.as_str())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate callback data".to_string(), interactive_reply.clone()))?;

            let callback_data: ButtonCallbackData = serde_json::from_str(data)
                .map_err(|_error| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate callback data must be a valid JSON string".to_string(), interactive_reply.clone()))?;

            // Update interaction time, carousel direction, and button index based on the callback data
            interaction_time = callback_data.timestamp_mark()
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate button timestamp mark".to_string(), interactive_reply.clone()))?;
            carousel_direction = callback_data.direction();
            button_index = callback_data.index();
        }

        // Create interaction type
        let interaction_type = InteractionType::new(text, button_index, carousel_direction);

        // Return the constructed WhatsAppUpdate instance
        Ok(Self::new(chat_id, interaction_time, interaction_type, update_id))
    }
}