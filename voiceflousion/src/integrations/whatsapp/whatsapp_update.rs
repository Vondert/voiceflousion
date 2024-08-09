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

        let chat_id = entry.get("id")
            .and_then(|id| id.as_str())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate entry id (chat id)".to_string(), entry.clone()))?;

        let value = entry.get("changes")
            .and_then(|changes_value| changes_value.as_array())
            .and_then(|changes_array| changes_array.first())
            .and_then(|changes| changes.get("value"))
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate value".to_string(), entry.clone()))?;

        let message = value.get("messages")
            .and_then(|messages_value| messages_value.as_array())
            .and_then(|messages_array| messages_array.first())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate messages".to_string(), value.clone()))?;

        let interaction_time = message.get("timestamp")
            .and_then(|date| date.as_str())
            .and_then(|date_str| date_str.parse::<i64>().ok())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate interaction timestamp".to_string(), message.clone()))?;

        let update_id = message.get("id")
            .and_then(|id| id.as_str())
            .map(|id| id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientUpdateConvertationError("WhatsAppUpdate update id".to_string(), message.clone()))?;

        let text = message.get("text")
            .and_then(|text_value| text_value.get("body"))
            .and_then(|text_message| text_message.as_str())
            .unwrap_or_default()
            .to_string();

        let interaction_type = InteractionType::new(text, None, None);

        Ok(Self::new(
            chat_id,
            interaction_time,
            interaction_type,
            update_id
        ))
    }
}