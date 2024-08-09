use std::ops::Deref;
use async_trait::async_trait;
use serde_json::{json, Map, Value};
use crate::core::base_structs::SenderBase;
use crate::core::subtypes::HttpClient;
use crate::integrations::telegram::TelegramResponder;
use crate::core::traits::{Responder, Sender};
use crate::core::voiceflow::VoiceflowBlock;
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::core::voiceflow::dialog_blocks::enums::{VoiceflowButtonActionType, VoiceflowButtonsOption};
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a sender for Telegram integration.
///
/// `TelegramSender` handles sending various types of messages (text, image, buttons, etc.)
/// to a Telegram client using the Telegram API.
pub struct TelegramSender {
    /// The base structure that provides core functionalities.
    sender_base: SenderBase
}

impl TelegramSender {

    /// The base URL for the Telegram API.
    const TELEGRAM_API_URL: &'static str = "https://api.telegram.org/bot";

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
    /// use voiceflousion::integrations::telegram::TelegramSender;
    ///
    /// let sender = TelegramSender::new(10, "api_key".to_string(), Some(120));
    /// let default_duration_sender = TelegramSender::new(10, "api_key".to_string(), None);
    /// ```
    pub fn new(max_sessions_per_moment: usize, api_key: String, connection_duration: Option<u64>) -> Self {
        Self {
            sender_base: SenderBase::new(max_sessions_per_moment, api_key, connection_duration)
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
    /// A `VoiceflousionResult` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    /// use voiceflousion::integrations::telegram::TelegramResponder;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     let cards = vec![VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None)];
    ///     let carousel = VoiceflowCarousel::new(cards, true);
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::new();
    ///     let message_id = String::new();
    ///     let response = sender.update_carousel(&carousel, 0, &chat_id, &message_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    pub async fn update_carousel(&self, carousel: &VoiceflowCarousel, index: usize, chat_id: &String, message_id: &String) -> VoiceflousionResult<TelegramResponder> {
        // Form the API URL for editing the message media via Telegram API
        let api_url = format!("{}{}/editMessageMedia", TelegramSender::TELEGRAM_API_URL, &self.api_key());

        // Get the card at the specified index, returning an error if the index is out of bounds
        let card = carousel.get(index).ok_or_else(|| VoiceflousionError::ClientRequestInvalidBodyError(
            "TelegramSender update_carousel".to_string(),
            format!("Provided card index {} is out of bounds of {} length", index, carousel.len()),
        ))?;

        // Convert the buttons from the card to the inline keyboard format
        let mut inline_keyboard: Vec<Vec<Value>> = if let Some(buttons) = card.buttons() {
            buttons_to_keyboard(buttons)
        } else {
            vec![]
        };


        // Add navigation buttons for the carousel
        let mut switch_buttons: Vec<Value> = Vec::new();
        if index > 0 {
            let carousel_prev = json!({
                "telegram_carousel_card_index": format!("{}", index - 1),
                "path": format!("path-t-carousel-{}", index - 1)
            });
            switch_buttons.push(json!({ "text": "<--", "callback_data":  carousel_prev.to_string()}));
        }
        if index < carousel.len() - 1 {
            let carousel_next = json!({
                "telegram_carousel_card_index": format!("{}", index + 1),
                "path": format!("path-t-carousel-{}", index + 1)
            });
            switch_buttons.push(json!({ "text": "-->", "callback_data": carousel_next.to_string() }));
        }
        inline_keyboard.push(switch_buttons);

        // Extract the title and description from the card
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        // Create the JSON body of the request for updating the carousel message
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

        // Send the POST request with the body to the Telegram API
        let response = self.http_client().post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender update_carousel".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a TelegramResponder
            TelegramResponder::from_response(response, VoiceflowBlock::Card(card.clone())).await
        } else {
            // Get the error text from the response and return an error
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender update_carousel".to_string(), error_text))
        }
    }
}

impl Deref for TelegramSender {
    type Target=SenderBase;

    fn deref(&self) -> &Self::Target {
        &self.sender_base
    }
}

#[async_trait]
impl Sender for TelegramSender{
    type SenderResponder = TelegramResponder;

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
    /// A `VoiceflousionResult` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::traits::Sender;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let cards = vec![VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None)];
    ///     let carousel = VoiceflowCarousel::new(cards, true);
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::new();
    ///     let client_id = String::new();
    ///     let text = VoiceflowText::new("Hello, World!".to_string());
    ///     let response = sender.send_text(&client_id, text, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_text(&self, _client_id: &String, text: VoiceflowText, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Form the API URL for sending the message via Telegram API
        let api_url = format!("{}{}/sendMessage", TelegramSender::TELEGRAM_API_URL, self.api_key());

        // Create the JSON body of the request containing chat_id and message text
        let body = json!({
            "chat_id": chat_id,
            "text": text.message(),
        });

        // Send the POST request with the body to the Telegram API
        let response = self.http_client().post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_text".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a TelegramResponder
            Self::SenderResponder::from_response(response, VoiceflowBlock::Text(text)).await
        } else {
            // Get the error text from the response and return an error
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
    /// A `VoiceflousionResult` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::traits::Sender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowImage;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::new();
    ///     let client_id = String::new();
    ///     let image = VoiceflowImage::new("https://example.com/image.jpg".to_string(), Some(100), Some(200));
    ///     let response = sender.send_image(&client_id, image, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_image(&self, _client_id: &String, image: VoiceflowImage, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Form the API URL for sending the image via Telegram API
        let api_url = format!("{}{}/sendPhoto", TelegramSender::TELEGRAM_API_URL, self.api_key());

        // Create the JSON body of the request containing chat_id and image URL
        let body = json!({
            "chat_id": chat_id,
            "photo": image.url(),
        });

        // Send the POST request with the body to the Telegram API
        let response = self.http_client().post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_image".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a TelegramResponder
            Self::SenderResponder::from_response(response, VoiceflowBlock::Image(image)).await
        } else {
            // Get the error text from the response and return an error
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
    /// A `VoiceflousionResult` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::traits::Sender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowButton, VoiceflowButtons};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     use serde_json::Value;
    /// let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::new();
    ///     let client_id = String::new();
    ///     let buttons = vec![VoiceflowButton::new("Click me".to_string(), "/path".to_string(), VoiceflowButtonActionType::Path, Value::Null)];
    ///     let voiceflow_buttons = VoiceflowButtons::new(buttons);
    ///     let response = sender.send_buttons(&client_id, voiceflow_buttons, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_buttons(&self, _client_id: &String, buttons: VoiceflowButtons, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Determine the API URL based on the button option (text or image)
        let api_url = match &buttons.option() {
            VoiceflowButtonsOption::Image(_) => format!("{}{}/sendPhoto", TelegramSender::TELEGRAM_API_URL, self.api_key()),
            _ => format!("{}{}/sendMessage", TelegramSender::TELEGRAM_API_URL, self.api_key()),
        };

        // Convert the buttons to the inline keyboard format for Telegram
        let inline_keyboard: Vec<Vec<Value>> = buttons_to_keyboard(&buttons);

        // Create the JSON body of the request based on the button option
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

        // Send the POST request with the body to the Telegram API
        let response = self.http_client().post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_buttons".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a TelegramResponder
            Self::SenderResponder::from_response(response, VoiceflowBlock::Buttons(buttons)).await
        } else {
            // Get the error text from the response and return an error
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
    /// A `VoiceflousionResult` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowCard;
    /// use voiceflousion::core::traits::Sender;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::new();
    ///     let client_id = String::new();
    ///     let message_id = String::new();
    ///     let response = sender.send_card(&client_id, card, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_card(&self, _client_id: &String, card: VoiceflowCard, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Extract the title and description from the card
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        // Convert the buttons from the card to the inline keyboard format
        let inline_keyboard: Vec<Vec<Value>> = if let Some(buttons) = card.buttons() {
            buttons_to_keyboard(buttons)
        } else {
            vec![]
        };

        // Initialize the API URL and request body for sending a message
        let mut api_url = format!("{}{}/sendMessage", TelegramSender::TELEGRAM_API_URL, self.api_key());
        let mut body = json!({
            "chat_id": chat_id,
            "text": format!("{}\n\n{}", title, description),
            "reply_markup": {
                "inline_keyboard": inline_keyboard,
            }
        });

        // If the card has an image URL, update the API URL and request body for sending a photo
        if let Some(url) = card.image_url() {
            api_url = format!("{}{}/sendPhoto", TelegramSender::TELEGRAM_API_URL, self.api_key());
            body = json!({
                "chat_id": chat_id,
                "photo": url,
                "caption": format!("{}\n\n{}", title, description),
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            });
        }

        // Send the POST request with the body to the Telegram API
        let response = self.http_client().post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_card".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a TelegramResponder
            Self::SenderResponder::from_response(response, VoiceflowBlock::Card(card)).await
        } else {
            // Get the error text from the response and return an error
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
    /// A `VoiceflousionResult` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    /// use voiceflousion::core::traits::Sender;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     use serde_json::Value::String;
    /// let cards = vec![VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None)];
    ///     let carousel = VoiceflowCarousel::new(cards, true);
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::new();
    ///     let client_id = String::new();
    ///     let message_id = String::new();
    ///     let response = sender.send_carousel(&client_id, carousel, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_carousel(&self, _client_id: &String, carousel: VoiceflowCarousel, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Check if the carousel is empty and return an error if it is
        if !carousel.is_full() {
            return Err(VoiceflousionError::ClientRequestInvalidBodyError("TelegramSender send_carousel".to_string(), "Provided carousel is empty!".to_string()));
        }

        // Form the API URL for sending the carousel via Telegram API
        let api_url = format!("{}{}/sendPhoto", TelegramSender::TELEGRAM_API_URL, self.api_key());

        // Get the first card from the carousel
        let card = carousel.get(0)
            .ok_or_else(|| VoiceflousionError::ClientRequestInvalidBodyError("TelegramSender send_carousel".to_string(), format!("Provided card index {} is out of bounds of {} length", 0, carousel.len())))?;

        // Extract the title and description from the card
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        // Convert the buttons from the card to the inline keyboard format
        let mut inline_keyboard: Vec<Vec<Value>> = if let Some(buttons) = card.buttons() {
            buttons_to_keyboard(buttons)
        } else {
            vec![]
        };

        // Add navigation buttons for the carousel if there are multiple cards
        let mut switch_buttons: Vec<Value> = Vec::new();
        if carousel.len() > 1 {
            let carousel_next = json!({
                "telegram_carousel_card_index": "1",
                "path": "path-t-carousel-1"
            });
            switch_buttons.push(json!({ "text": "-->", "callback_data": carousel_next.to_string()}));
        }
        inline_keyboard.push(switch_buttons);

        // Create the JSON body of the request based on the card's image URL
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

        // Send the POST request with the body to the Telegram API
        let response = self.http_client().post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_carousel".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Convert the response to a TelegramResponder
            Self::SenderResponder::from_response(response, VoiceflowBlock::Carousel(carousel)).await
        } else {
            // Get the error text from the response and return an error
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
    // Map each button to a JSON value based on its action type
    buttons.iter().map(|b| {
        let mut callback_data = b.payload().as_object().cloned().unwrap_or_else(Map::new);
        callback_data.insert("path".to_string(), Value::String(b.path().clone()));
        let callback_data = Value::Object(callback_data);

        match &b.action_type() {
            VoiceflowButtonActionType::OpenUrl(url) => {
                let url = if url.is_empty(){
                    "empty"
                }
                else{
                    url
                };
                json!({ "text": b.name(), "url": url, "callback_data": callback_data.to_string() })
            },
            VoiceflowButtonActionType::Path => json!({ "text": b.name(), "callback_data": callback_data.to_string() }),
        }
    }).map(|key| vec![key]).collect()
}