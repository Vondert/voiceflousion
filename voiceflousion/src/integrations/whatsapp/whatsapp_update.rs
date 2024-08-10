use std::ops::Deref;
use serde_json::Value;
use crate::core::base_structs::UpdateBase;
use crate::core::subtypes::InteractionType;
use crate::core::traits::Update;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::telegram::TelegramUpdate;

#[derive(Debug)]
pub struct WhatsAppUpdate{
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
    pub fn new(chat_id: String, interaction_time: i64, interaction_type: InteractionType, update_id: String) -> Self {
        Self {
            update_base: UpdateBase::new(chat_id, interaction_time, interaction_type, update_id),
        }
    }
}

impl Update for WhatsAppUpdate{
    fn from_request_body(body: Value) -> VoiceflousionResult<Self> {
        let entry = body.get("entry")
            .and_then(|entry_value| entry_value.as_array())
            .and_then(|entry_array| entry_array.first())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate entry".to_string(), body.clone()))?;

        // let chat_id = entry.get("id")
        //     .and_then(|id| id.as_str())
        //     .map(|id| id.to_string())
        //     .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate entry id (chat id)".to_string(), entry.clone()))?;

        let value = entry["changes"][0].get("value")
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate value".to_string(), entry.clone()))?;

        let message = value.get("messages")
            .and_then(|messages_value| messages_value.as_array())
            .and_then(|messages_array| messages_array.first())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate messages".to_string(), value.clone()))?;

        let chat_id = message.get("from")
             .and_then(|from| from.as_str())
             .map(|from| from.to_string())
             .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate message from (chat id)".to_string(), message.clone()))?;

        let interaction_time = message.get("timestamp")
            .and_then(|date| date.as_str())
            .and_then(|date_str| date_str.parse::<i64>().ok())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate interaction timestamp".to_string(), message.clone()))?;

        let update_id = message.get("id")
            .and_then(|id| id.as_str())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update id".to_string(), message.clone()))?;

        let message_type = message["type"].as_str()
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update type".to_string(), message.clone()))?;

        let is_message = if message_type == "text"{
            true
        }
        else{
            false
        };

        let mut text: String = String::new();
        let mut path = None;
        let mut callback_data = None;

        if is_message{
            text = message["text"].get("body")
                .and_then(|body| body.as_str())
                .map(|text_str| text_str.to_string())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update message text".to_string(), message.clone()))?;
        }
        else{
            let button_reply = message["interactive"].get("button_reply")
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update button reply".to_string(), message.clone()))?;

            text = button_reply.get("title").and_then(|text_value| text_value.as_str())
                .map(|text_str| text_str.to_string())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update button title".to_string(), button_reply.clone()))?;

            let data = button_reply.get("id")
                .and_then(|data| data.as_str())
                .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate callback data".to_string(), button_reply.clone()))?;

            let mut deserialized_data: Value = serde_json::from_str(data)
                .map_err(|_error| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate callback data must be a valid JSON string".to_string(), button_reply.clone()))?;

            if let Some(mut_data) = deserialized_data.as_object_mut(){
                path = mut_data.remove("path")
                    .and_then(|value_path| value_path.as_str().map(|s| s.to_string()));
            }
            callback_data = Some(deserialized_data);
        }

        let interaction_type = InteractionType::new(text, path, callback_data);

        Ok(Self::new(
            chat_id,
            interaction_time,
            interaction_type,
            update_id
        ))
    }
}