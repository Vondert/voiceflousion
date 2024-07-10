use async_trait::async_trait;
use reqwest::Response;
use serde_json::{json, Value};
use crate::integrations::utils::SenderHttpClient;
use crate::integrations::utils::traits::Sender;
use crate::voiceflow::VoiceflousionError;
use crate::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::voiceflow::dialog_blocks::enums::{VoiceflowButtonActionType, VoiceflowButtonsOption};

static TELEGRAM_API_URL: &str = "https://api.telegram.org/bot";
pub struct TelegramSender{
    sender_http_client: SenderHttpClient,
    api_key: String
}
impl TelegramSender{
    pub fn new(max_sessions_per_moment: usize, api_key: String) -> Self{
        Self{
            sender_http_client: SenderHttpClient::new(max_sessions_per_moment),
            api_key
        }
    }
    pub async fn update_carousel(&self, carousel: &VoiceflowCarousel, index: usize, chat_id: &String, message_id: &String) -> Result<Response, VoiceflousionError>{
        let api_url = format!("{}{}/editMessageMedia", TELEGRAM_API_URL, &self.api_key);
        let card = carousel.get(index).ok_or_else(|| VoiceflousionError::RequestError("Card out of bounds".to_string()))?;
        let mut inline_keyboard: Vec<Vec<Value>> = if let Some(buttons) = card.buttons(){
            buttons_to_keyboard(buttons)
        }
        else{
            vec![]
        };
        let mut switch_buttons: Vec<Value> = Vec::new();
        if index < carousel.len() - 1{
            switch_buttons.push(json!({ "text": "-->", "callback_data": format!("c_{}", index + 1) }));
        }
        if index > 0 {
            switch_buttons.push( json!({ "text": "<--", "callback_data": format!("c_{}", index - 1) }));
        }
        inline_keyboard.push(switch_buttons);

        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());
        let body = json!({
            "chat_id": chat_id,
            "message_id": message_id,
            "media": {
                "type": "photo",
                "media": card.image_url(),
                "caption": format!("{}\n\n{}", title, description),
            },
            "reply_markup": {
                "inline_keyboard": inline_keyboard,
            }
        });
        let response = self.sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::RequestError(e.to_string()))?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::RequestError(error_text))
        }
    }
}
#[async_trait]
impl Sender for TelegramSender{
    fn sender_http_client(&self) -> &SenderHttpClient {
        &self.sender_http_client
    }

    fn api_key(&self) -> &String {
        &self.api_key
    }

    async fn send_text(&self, text: &VoiceflowText, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError> {
        let api_url = format!("{}{}/sendMessage", TELEGRAM_API_URL, api_key);
        let body = json!({
            "chat_id": chat_id,
            "text": text.message(),
        });

        let response = sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|error| VoiceflousionError::RequestError(error.to_string()))?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::RequestError(error_text))
        }
    }

    async fn send_image(&self, image: &VoiceflowImage, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError>{
        let api_url = format!("{}{}/sendPhoto", TELEGRAM_API_URL, api_key);
        let body = json!({
            "chat_id": chat_id,
            "photo": image.url(),
        });

        let response = sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|error| VoiceflousionError::RequestError(error.to_string()))?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::RequestError(error_text))
        }
    }

    async fn send_buttons(&self, buttons: &VoiceflowButtons, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError> {
        let api_url = match &buttons.option() {
            VoiceflowButtonsOption::Image(_) => format!("{}{}/sendPhoto", TELEGRAM_API_URL, api_key),
            _ => format!("{}{}/sendMessage", TELEGRAM_API_URL, api_key),
        };

        let inline_keyboard: Vec<Vec<Value>> = buttons_to_keyboard(buttons);


        let body = match &buttons.option() {
            VoiceflowButtonsOption::Text(text) => json!({
                "chat_id": chat_id,
                "text": text.message(),
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            }),
            VoiceflowButtonsOption::Image(image) => json!({
                "chat_id": chat_id,
                "photo": image.url(),
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            }),
            VoiceflowButtonsOption::Empty => json!({
                "chat_id": chat_id,
                "text": "",
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            }),
        };

        let response = sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|error| VoiceflousionError::RequestError(error.to_string()))?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::RequestError(error_text))
        }
    }

    async fn send_card(&self, card: &VoiceflowCard, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError> {
        let inline_keyboard: Vec<Vec<Value>> = if let Some(buttons) = card.buttons(){
            buttons_to_keyboard(buttons)
        }
        else{
            vec![]
        };
        let mut api_url = String::new();
        let mut body: Value = json!({});

        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());
        match card.image_url() {
            Some(url) => {
                api_url = format!("{}{}/sendPhoto", TELEGRAM_API_URL, api_key);
                body = json!({
                    "chat_id": chat_id,
                    "photo": url,
                    "caption": format!("{}\n\n{}", title, description),
                    "reply_markup": {
                        "inline_keyboard": inline_keyboard,
                    }
                });
            },
            None => {
                api_url = format!("{}{}/sendMessage", TELEGRAM_API_URL, api_key);
                body = json!({
                    "chat_id": chat_id,
                    "text": format!("{}\n\n{}", title, description),
                    "reply_markup": {
                        "inline_keyboard": inline_keyboard,
                    }
                })
            },
        };

        let response = sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|error| VoiceflousionError::RequestError(error.to_string()))?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::RequestError(error_text))
        }
    }
    async fn send_carousel(&self, carousel: &VoiceflowCarousel, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError> {
        let card = carousel.get(0).ok_or_else(|| VoiceflousionError::RequestError("Card out of bounds".to_string()))?;
        self.send_card(card, chat_id, sender_http_client, api_key).await
    }
}
fn buttons_to_keyboard(buttons: &VoiceflowButtons) -> Vec<Vec<Value>>{
    println!("{:?}", buttons);
    buttons.iter().map(|b| {
        match &b.action_type() {
            VoiceflowButtonActionType::OpenUrl(url) => {
                let url = if url.is_empty(){
                    "empty"
                }
                else{
                    url
                };
                json!({ "text": b.name(), "url": url, "callback_data": b.path() })
            },
            VoiceflowButtonActionType::Path => json!({ "text": b.name(), "callback_data": b.path() }),
        }
    }).map(|key| vec![key]).collect()
}