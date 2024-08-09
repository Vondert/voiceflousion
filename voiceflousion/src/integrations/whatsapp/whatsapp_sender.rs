use std::ops::Deref;
use async_trait::async_trait;
use serde_json::{json, Map, Value};
use crate::core::base_structs::SenderBase;
use crate::core::subtypes::HttpClient;
use crate::core::traits::{Responder, Sender};
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::core::voiceflow::dialog_blocks::enums::{VoiceflowButtonActionType, VoiceflowButtonsOption};
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::whatsapp::whatsapp_responder::WhatsAppResponder;



pub struct WhatsAppSender{
    /// The base structure that provides core functionalities.
    sender_base: SenderBase
}

impl Deref for WhatsAppSender {
    type Target = SenderBase;

    fn deref(&self) -> &Self::Target {
        &self.sender_base
    }
}

impl WhatsAppSender{

    const WHATSAPP_API_URL: &'static str = "https://graph.facebook.com/v20.0/";

    pub fn new(max_sessions_per_moment: usize, api_key: String, connection_duration: Option<u64>) -> Self {
        Self {
            sender_base: SenderBase::new(max_sessions_per_moment, api_key, connection_duration)
        }
    }
}

#[async_trait]
impl Sender for WhatsAppSender{
    type SenderResponder = WhatsAppResponder;

    async fn send_text(&self, client_id: &String, text: VoiceflowText, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Form the API URL for sending the message via WhatsApp API
        let api_url = format!("{}{}/messages", WhatsAppSender::WHATSAPP_API_URL, client_id);

        // Create the JSON body of the request containing chat_id and message text
        let body = json!({
            "messaging_product": "whatsapp",
            "to": chat_id,
            "type": "text",
            "text": {
                "body": text.message(),
            },
        });

        // Send the POST request with the body to the WhatsApp API
        let response = self.http_client()
            .post(&api_url)
            .json(&body)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("WhatsAppSender send_text".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a WhatsAppResponder
            Self::SenderResponder::from_response(response, VoiceflowBlock::Text(text)).await
        } else {
            // Get the error text from the response and return an error
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("WhatsAppSender send_text".to_string(), error_text))
        }
    }

    async fn send_image(&self, client_id: &String, image: VoiceflowImage, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Form the API URL for sending the image via WhatsApp API
        let api_url = format!("{}{}/messages", WhatsAppSender::WHATSAPP_API_URL, client_id);

        // Create the JSON body of the request containing chat_id and image URL
        let body = json!({
        "messaging_product": "whatsapp",
        "to": chat_id,
        "type": "image",
        "image": {
            "link": image.url(), // URL of the image
        }
    });

        // Send the POST request with the body to the WhatsApp API
        let response = self.http_client()
            .post(&api_url)
            .json(&body)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("WhatsAppSender send_image".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a WhatsAppResponder
            Self::SenderResponder::from_response(response, VoiceflowBlock::Image(image)).await
        } else {
            // Get the error text from the response and return an error
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("WhatsAppSender send_image".to_string(), error_text))
        }
    }

    async fn send_buttons(&self, client_id: &String, buttons: VoiceflowButtons, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Form the API URL for sending the message via WhatsApp API
        let api_url = format!("{}{}/messages", WhatsAppSender::WHATSAPP_API_URL, client_id);

        // Convert VoiceflowButtons to WhatsApp interactive buttons with callback data (id)
        let interactive_buttons: Vec<Value> = buttons_to_keyboard(&buttons);

        // Create the JSON body of the request containing chat_id and buttons
        let body = match &buttons.option() {
            VoiceflowButtonsOption::Text(text) => json!({
                "messaging_product": "whatsapp",
                "to": chat_id,
                "type": "interactive",
                "interactive": {
                    "type": "button",
                    "body": {
                        "text": text.message(),
                    },
                    "action": {
                        "buttons": interactive_buttons,
                    }
                }
            }),
            VoiceflowButtonsOption::Image(image) => json!({
                "messaging_product": "whatsapp",
                "to": chat_id,
                "type": "image",
                "image": {
                    "link": image.url(),
                },
                "interactive": {
                    "type": "button",
                    "action": {
                        "buttons": interactive_buttons,
                    }
                }
            }),
            VoiceflowButtonsOption::Empty => json!({
                "messaging_product": "whatsapp",
                "to": chat_id,
                "type": "interactive",
                "interactive": {
                    "type": "button",
                    "body": {
                        "text": "",
                    },
                    "action": {
                        "buttons": interactive_buttons,
                    }
                }
            }),
        };

        // Send the POST request with the body to the WhatsApp API
        let response = self.http_client()
            .post(&api_url)
            .json(&body)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("WhatsAppSender send_buttons".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a WhatsAppResponder
            Self::SenderResponder::from_response(response, VoiceflowBlock::Buttons(buttons)).await
        } else {
            // Get the error text from the response and return an error
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("WhatsAppSender send_buttons".to_string(), error_text))
        }
    }

    async fn send_card(&self, client_id: &String, card: VoiceflowCard, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_carousel(&self, client_id: &String, carousel: VoiceflowCarousel, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }
}

fn buttons_to_keyboard(buttons: &VoiceflowButtons) -> Vec<Value> {
    buttons.iter().map(|b| {
        let mut callback_data = b.payload().as_object().cloned().unwrap_or_else(Map::new);
        callback_data.insert("path".to_string(), Value::String(b.path().clone()));

        // Convert callback_data to a JSON string
        let callback_data_string = serde_json::to_string(&callback_data).unwrap_or_else(|_| "".to_string());

        match &b.action_type() {
            VoiceflowButtonActionType::OpenUrl(url) => {
                let url = if url.is_empty() { "empty" } else { url };
                json!({
                    "type": "url",
                    "reply": {
                        "id": callback_data_string,
                        "title": b.name(),
                        "url": url,
                    }
                })
            },
            VoiceflowButtonActionType::Path => {
                json!({
                    "type": "reply",
                    "reply": {
                        "id": callback_data_string,
                        "title": b.name(),
                    }
                })
            },
        }
    }).collect()
}