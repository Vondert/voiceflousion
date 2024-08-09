use std::fmt::Debug;
use std::ops::Deref;
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Response;
use serde_json::Value;
use crate::core::base_structs::ResponderBase;
use crate::core::traits::Responder;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::VoiceflousionError::VoiceflowRequestError;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

#[derive(Debug)]
pub struct WhatsAppResponder{
    ///
    wa_id: String,
    /// The base structure that provides core functionalities.
    responder_base: ResponderBase
}

impl Deref for WhatsAppResponder {
    type Target = ResponderBase;

    fn deref(&self) -> &Self::Target {
        &self.responder_base
    }
}

#[async_trait]
impl Responder for WhatsAppResponder{
    async fn from_response(response: Response, content: VoiceflowBlock) -> VoiceflousionResult<Self> {
        let timestamp = Utc::now().timestamp();

        let json: Value = response.json().await.map_err(|e| VoiceflousionError::ClientResponseReadingError("WhatsAppResponder".to_string(), e.to_string()))?;

        let wa_id = json["contacts"][0]["wa_id"]
            .as_str()
            .map(|wa_id| wa_id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientResponseReadingError("WhatsAppResponder wa_id".to_string(), json.to_string()))?;
        let message_id = json["messages"][0]["id"]
            .as_str()
            .map(|message_id| message_id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientResponseReadingError("WhatsAppResponder message_id".to_string(), json.to_string()))?;

        Ok(Self{
            wa_id,
            responder_base: ResponderBase::new(message_id, content, timestamp)
        })
    }
}