use async_trait::async_trait;
use serde_json::{json, Value};
use crate::core::subtypes::HttpClient;
use crate::integrations::telegram::TelegramResponder;
use crate::core::traits::{Responder, Sender};
use crate::core::voiceflow::{VoiceflousionError, VoiceflowBlock};
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::core::voiceflow::dialog_blocks::enums::{VoiceflowButtonActionType, VoiceflowButtonsOption};

/// The base URL for the Telegram API.
static TELEGRAM_API_URL: &str = "https://api.telegram.org/bot";

/// Represents a sender for Telegram integration.
///
/// `TelegramSender` handles sending various types of messages (text, image, buttons, etc.)
/// to a Telegram client using the Telegram API.
pub struct TelegramSender {
    /// The HTTP client for sending requests.
    http_client: HttpClient,
    /// The API key for authenticating with the Telegram API.
    api_key: String,
}

impl TelegramSender {
    /// Creates a new `TelegramSender`.
    ///
    /// # Parameters
    ///
    /// * `max_sessions_per_moment` - The maximum number of idle connections per host.
    /// * `api_key` - The API key for authenticating with the Telegram API.
    /// * `connection_duration` - The optional duration for which sessions can remain idle (in seconds).
    ///
    /// # Returns
    ///
    /// A new instance of `TelegramSender`.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string(), Some(120));
    /// let default_duration_sender = TelegramSender::new(10, "api_key".to_string(), None);
    /// ```
    pub fn new(max_sessions_per_moment: usize, api_key: String, connection_duration: Option<u64>) -> Self {
        Self {
            http_client: HttpClient::new(max_sessions_per_moment, connection_duration),
            api_key,
        }
    }

    /// Updates a carousel message in a chat.
    ///
    /// # Parameters
    ///
    /// * `carousel` - The carousel to update.
    /// * `index` - The index of the card to display.
    /// * `chat_id` - The chat ID of the recipient.
    /// * `message_id` - The ID of the message to update.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string());
    /// let response = sender.update_carousel(&carousel, 0, &chat_id, &message_id).await?;
    /// ```
    pub async fn update_carousel(&self, carousel: &VoiceflowCarousel, index: usize, chat_id: &String, message_id: &String) -> Result<TelegramResponder, VoiceflousionError> {
        let api_url = format!("{}{}/editMessageMedia", TELEGRAM_API_URL, &self.api_key);
        let card = carousel.get(index).ok_or_else(|| VoiceflousionError::ClientRequestInvalidBodyError(
            "TelegramSender update_carousel".to_string(),
            format!("Provided card index {} is out of bounds of {} length", index, carousel.len()),
        ))?;

        let mut inline_keyboard: Vec<Vec<Value>> = if let Some(buttons) = card.buttons() {
            buttons_to_keyboard(buttons)
        } else {
            vec![]
        };

        let mut switch_buttons: Vec<Value> = Vec::new();
        if index > 0 {
            switch_buttons.push(json!({ "text": "<--", "callback_data": format!("c_{}", index - 1) }));
        }
        if index < carousel.len() - 1 {
            switch_buttons.push(json!({ "text": "-->", "callback_data": format!("c_{}", index + 1) }));
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
        let response = self.http_client.post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender update_carousel".to_string(), e.to_string()))?;

        if response.status().is_success() {
            TelegramResponder::from_response(response, VoiceflowBlock::Card(card.clone())).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender update_carousel".to_string(), error_text))
        }
    }
}

#[async_trait]
impl Sender for TelegramSender{
    type SenderResponder = TelegramResponder;

    /// Returns a reference to the HTTP client.
    ///
    /// # Returns
    ///
    /// A reference to the `HttpClient`.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string());
    /// let http_client = &sender.http_client();
    /// ```
    fn http_client(&self) -> &HttpClient {
        &self.http_client
    }

    /// Returns a reference to the API key.
    ///
    /// # Returns
    ///
    /// A reference to the API key string.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string());
    /// let api_key = &sender.api_key();
    /// ```
    fn api_key(&self) -> &String {
        &self.api_key
    }

    /// Sends a text message to a chat.
    ///
    /// # Parameters
    ///
    /// * `text` - The text message to send.
    /// * `chat_id` - The chat ID of the recipient.
    /// * `sender_http_client` - The HTTP client for sending requests.
    /// * `api_key` - The API key for authenticating with the Telegram API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string());
    /// let chat_id = "chat_id_value".to_string();
    /// let response = sender.send_text(text, &chat_id, &sender.http_client, &sender.api_key).await?;
    /// ```
    async fn send_text(&self, text: VoiceflowText, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
        let api_url = format!("{}{}/sendMessage", TELEGRAM_API_URL, api_key);
        let body = json!({
            "chat_id": chat_id,
            "text": text.message(),
        });

        let response = sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_text".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Self::SenderResponder::from_response(response, VoiceflowBlock::Text(text)).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_text".to_string(), error_text))
        }
    }

    /// Sends an image message to a chat.
    ///
    /// # Parameters
    ///
    /// * `image` - The image message to send.
    /// * `chat_id` - The chat ID of the recipient.
    /// * `sender_http_client` - The HTTP client for sending requests.
    /// * `api_key` - The API key for authenticating with the Telegram API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string());
    /// let chat_id = "chat_id_value".to_string();
    /// let response = sender.send_image(image, &chat_id, &sender.http_client, &sender.api_key).await?;
    /// ```
    async fn send_image(&self, image: VoiceflowImage, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError>{
        let api_url = format!("{}{}/sendPhoto", TELEGRAM_API_URL, api_key);
        let body = json!({
            "chat_id": chat_id,
            "photo": image.url(),
        });

        let response = sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_image".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Self::SenderResponder::from_response(response, VoiceflowBlock::Image(image)).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_image".to_string(), error_text))
        }
    }

    /// Sends a buttons message to a chat.
    ///
    /// # Parameters
    ///
    /// * `buttons` - The buttons message to send.
    /// * `chat_id` - The chat ID of the recipient.
    /// * `sender_http_client` - The HTTP client for sending requests.
    /// * `api_key` - The API key for authenticating with the Telegram API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string());
    /// let chat_id = "chat_id_value".to_string();
    /// let response = sender.send_buttons(buttons, &chat_id, &sender.http_client, &sender.api_key).await?;
    /// ```
    async fn send_buttons(&self, buttons: VoiceflowButtons, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
        let api_url = match &buttons.option() {
            VoiceflowButtonsOption::Image(_) => format!("{}{}/sendPhoto", TELEGRAM_API_URL, api_key),
            _ => format!("{}{}/sendMessage", TELEGRAM_API_URL, api_key),
        };

        let inline_keyboard: Vec<Vec<Value>> = buttons_to_keyboard(&buttons);


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
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_buttons".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Self::SenderResponder::from_response(response, VoiceflowBlock::Buttons(buttons)).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_buttons".to_string(), error_text))
        }
    }

    /// Sends a card message to a chat.
    ///
    /// # Parameters
    ///
    /// * `card` - The card message to send.
    /// * `chat_id` - The chat ID of the recipient.
    /// * `sender_http_client` - The HTTP client for sending requests.
    /// * `api_key` - The API key for authenticating with the Telegram API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string());
    /// let chat_id = "chat_id_value".to_string();
    /// let response = sender.send_card(card, &chat_id, &sender.http_client, &sender.api_key).await?;
    /// ```
    async fn send_card(&self, card: VoiceflowCard, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());
        let inline_keyboard: Vec<Vec<Value>> = if let Some(buttons) = card.buttons(){
            buttons_to_keyboard(buttons)
        }
        else{
            vec![]
        };


        let mut api_url = format!("{}{}/sendMessage", TELEGRAM_API_URL, api_key);
        let mut body = json!({
                    "chat_id": chat_id,
                    "text": format!("{}\n\n{}", title, description),
                    "reply_markup": {
                        "inline_keyboard": inline_keyboard,
                    }
                });

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
            None => {}
        };

        let response = sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_card".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Self::SenderResponder::from_response(response, VoiceflowBlock::Card(card)).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_card".to_string(), error_text))
        }
    }

    /// Sends a carousel message to a chat.
    ///
    /// # Parameters
    ///
    /// * `carousel` - The carousel message to send.
    /// * `chat_id` - The chat ID of the recipient.
    /// * `sender_http_client` - The HTTP client for sending requests.
    /// * `api_key` - The API key for authenticating with the Telegram API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let sender = TelegramSender::new(10, "api_key".to_string());
    /// let chat_id = "chat_id_value".to_string();
    /// let response = sender.send_carousel(carousel, &chat_id, &sender.http_client, &sender.api_key).await?;
    /// ```
    async fn send_carousel(&self, carousel: VoiceflowCarousel, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
        if !carousel.is_full(){
            return Err(VoiceflousionError::ClientRequestInvalidBodyError("TelegramSender send_carousel".to_string(), "Provided carousel is empty!".to_string()));
        }
        let api_url = format!("{}{}/sendPhoto", TELEGRAM_API_URL, api_key);
        let card = carousel.get(0)
            .ok_or_else(|| VoiceflousionError::ClientRequestInvalidBodyError("TelegramSender send_carousel".to_string(), format!("Provided card index {} is out of bounds of {} length", 0, carousel.len())))?;

        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        let mut inline_keyboard: Vec<Vec<Value>> = if let Some(buttons) = card.buttons(){
            buttons_to_keyboard(buttons)
        }
        else{
            vec![]
        };
        let mut switch_buttons: Vec<Value> = Vec::new();
        if carousel.len() > 1{
            switch_buttons.push(json!({ "text": "-->", "callback_data": format!("c_{}", 1) }));
        }
        inline_keyboard.push(switch_buttons);

        let body = match card.image_url() {
            Some(url) => {
                json!({
                    "chat_id": chat_id,
                    "photo": url,
                    "caption": format!("{}\n\n{}", title, description),
                    "reply_markup": {
                        "inline_keyboard": inline_keyboard,
                    }
                })
            },
            None => json!({})
        };

        let response = sender_http_client.post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_carousel".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Self::SenderResponder::from_response(response, VoiceflowBlock::Carousel(carousel)).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_carousel".to_string(), error_text))
        }
    }
}

/// Converts `VoiceflowButtons` to a keyboard layout for Telegram inline keyboard.
///
/// # Parameters
///
/// * `buttons` - The `VoiceflowButtons` to convert.
///
/// # Returns
///
/// A vector of vectors containing the keyboard layout in JSON format.
fn buttons_to_keyboard(buttons: &VoiceflowButtons) -> Vec<Vec<Value>>{
    //println!("{:?}", buttons);
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