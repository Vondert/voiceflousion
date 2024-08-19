use std::fmt::Debug;
use std::ops::Deref;
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Response;
use serde_json::Value;
use crate::core::base_structs::ResponderBase;
use crate::core::traits::Responder;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a responder for WhatsApp integration.
///
/// `WhatsAppResponder` processes the response from the WhatsApp API, extracting relevant
/// information such as message ID and timestamp, and provides a structured response.
#[derive(Debug)]
pub struct WhatsAppResponder {
    /// The base structure that provides core functionalities.
    responder_base: ResponderBase,
}

impl Deref for WhatsAppResponder {
    type Target = ResponderBase;

    fn deref(&self) -> &Self::Target {
        &self.responder_base
    }
}

#[async_trait]
impl Responder for WhatsAppResponder{

    /// Creates a new `WhatsAppResponder` instance from an HTTP response and the associated `VoiceflowBlock`.
    ///
    /// This method extracts necessary data such as `wa_id`, `message_id`, and `timestamp` from the response
    /// to create an instance of `WhatsAppResponder`.
    ///
    /// # Parameters
    ///
    /// * `response` - The HTTP response from the WhatsApp API.
    /// * `content` - The `VoiceflowBlock` associated with the response, used to extract the relevant timestamp.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing the `WhatsAppResponder` instance or an error if the response
    /// processing fails.
    ///
    /// # Errors
    ///
    /// This function returns a `VoiceflousionError` if the response body cannot be read or if expected fields
    /// like `wa_id` or `message_id` are missing.
    async fn from_response(response: Response, content: VoiceflowBlock) -> VoiceflousionResult<Self> {
        // Determine the timestamp based on the content type.
        let timestamp = match &content{
            VoiceflowBlock::Buttons(buttons) => {
                buttons.mark()
            },
            VoiceflowBlock::Card(card) => {
                if let Some(buttons) = card.buttons(){
                    buttons.mark()
                }
                else{
                    Utc::now().timestamp()
                }
            },
            VoiceflowBlock::Carousel(carousel) => {
                carousel.get_selected_mark()
            },
            _ => Utc::now().timestamp()
        };

        // Parse the response JSON.
        let json: Value = response.json().await.map_err(|e| VoiceflousionError::ClientResponseReadingError("WhatsAppResponder".to_string(), e.to_string()))?;

        // Extract the WhatsApp ID (wa_id) from the response.
        let wa_id = json["contacts"][0]["wa_id"]
            .as_str()
            .map(|wa_id| wa_id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientResponseReadingError("WhatsAppResponder wa_id".to_string(), json.to_string()))?;

        // Extract the message ID from the response.
        let message_id = json["messages"][0]["id"]
            .as_str()
            .map(|message_id| message_id.to_string())
            .ok_or_else(|| VoiceflousionError::ClientResponseReadingError("WhatsAppResponder message_id".to_string(), json.to_string()))?;

        // Construct the WhatsAppResponder with the extracted data.
        Ok(Self{
            responder_base: ResponderBase::new(message_id, wa_id, content, timestamp)
        })
    }
}