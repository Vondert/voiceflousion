use std::ops::Deref;
use async_trait::async_trait;
use serde_json::{json, Map, Number, Value};
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
                "link": image.url(),
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

        // Convert VoiceflowButtons to WhatsApp list rows
        let interactive_rows: Vec<Value> = buttons_to_list_rows(&buttons);

        // Create the JSON body of the request containing chat_id and buttons
        let body = match &buttons.option() {
            VoiceflowButtonsOption::Text(text) => json!({
                "messaging_product": "whatsapp",
                "to": chat_id,
                "type": "interactive",
                "interactive": {
                    "type": "list",
                    "body": {
                        "text": text.message(),
                    },
                    "action": {
                    "button": "👇",
                    "sections": [
                        {
                            "title": "Buttons",
                            "rows": interactive_rows,
                        }
                    ]
                }
            }}),
            VoiceflowButtonsOption::Empty =>  panic!("Buttons with empty text field caught!"),
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
        // Form the API URL for sending the message via WhatsApp API
        let api_url = format!("{}{}/messages", WhatsAppSender::WHATSAPP_API_URL, client_id);

        // Convert VoiceflowButtons to WhatsApp list rows
        let mut interactive_rows: Vec<Value> = if let Some(buttons) = card.buttons() {
            buttons_to_list_rows(buttons)
            //buttons_to_keyboard(buttons)
        } else {
            vec![]
        };

        if interactive_rows.len() > 10{
            interactive_rows = interactive_rows[0..10].to_owned();
        }

        // Extract the title and description from the card
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        let text = if title.is_empty() && description.is_empty(){
            String::new()
        }
        else{
            format!("{}\n\n{}", title, description)
        };

        let mut card_parts:Vec<Value> = Vec::with_capacity(2);

        if interactive_rows.is_empty(){
            let mut body = json!({
                "messaging_product": "whatsapp",
                "to": chat_id
            });

            let body_mut = body.as_object_mut().unwrap();

            if let Some(url) = card.image_url(){
                body_mut.insert("type".to_string(), json!("image"));

                let mut image_object = serde_json::Map::new();
                image_object.insert("link".to_string(), json!(url));
                if !text.is_empty() {
                    image_object.insert("caption".to_string(), json!(text));
                }

                body_mut.insert("image".to_string(), json!(image_object));
            }
            else{
                body_mut.insert("type".to_string(), json!("text"));
                body_mut.insert("text".to_string(), json!({
                    "body": text,
                }));
            }
            card_parts.push(body);
        }
        else{
            if let Some(url) = card.image_url() {
                let image_body = json!({
                    "messaging_product": "whatsapp",
                    "to": chat_id,
                    "type": "image",
                    "image": {
                        "link": url,
                    }
                });
                card_parts.push(image_body);
            }
            let body = json!({
                "messaging_product": "whatsapp",
                "to": chat_id,
                "type": "interactive",
                "interactive": {
                    "type": "list",
                    "body": {
                        "text": text
                    },
                    "action": {
                        "button": "👇",
                        "sections": [
                            {
                                "title": "Buttons",
                                "rows": interactive_rows,
                            }
                        ]
                    }
                }
            });
            card_parts.push(body);
        };

        let mut last_response = None;

        for body in card_parts {
            let response = self.http_client()
                .post(&api_url)
                .json(&body)
                .header("Authorization", format!("Bearer {}", self.api_key()))
                .send()
                .await.map_err(|e| VoiceflousionError::ClientRequestError("WhatsAppSender send_card".to_string(), e.to_string()))?;

            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(VoiceflousionError::ClientRequestError("WhatsAppSender send_card".to_string(), error_text))
            }
            last_response = Some(response);
        }
        Self::SenderResponder::from_response(last_response.expect("Empty response"), VoiceflowBlock::Card(card)).await
    }

    async fn send_carousel(&self, client_id: &String, carousel: VoiceflowCarousel, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }
}

fn buttons_to_list_rows(buttons: &VoiceflowButtons) -> Vec<Value> {
    let mark = buttons.mark();
    buttons.iter().map(|b| {
        let mut callback_data = b.payload().as_object().cloned().unwrap_or_else(Map::new);
        callback_data.insert("mark".to_string(), Value::from(mark));

        // Convert callback_data to a JSON string
        let callback_data_string = serde_json::to_string(&callback_data).unwrap_or_else(|_| "".to_string());

        json!({
            "id": callback_data_string,
            "title": b.name(),
            "description": ""
        })
    }).collect()
}