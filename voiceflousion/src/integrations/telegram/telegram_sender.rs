use std::ops::Deref;
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Response;
use serde_json::Value;
use crate::core::base_structs::SenderBase;
use crate::integrations::telegram::TelegramResponder;
use crate::core::traits::{Responder, Sender};
use crate::core::voiceflow::VoiceflowBlock;
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::telegram::utils::TelegramSerializer;

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

    /// Sends a message to the Telegram API.
    ///
    /// # Parameters
    ///
    /// * `api_url` - The API endpoint URL.
    /// * `body` - The JSON body of the message.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `Response` or a `VoiceflousionError` if the request fails.
    async fn send_message(&self, api_url: &str, body: Value) -> VoiceflousionResult<Response> {
        self.http_client()
            .post(api_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender send_message".to_string(), e.to_string()))
    }


    /// Updates a carousel message in a chat.
    ///
    /// # Parameters
    ///
    /// * `carousel` - The carousel to update.
    /// * `direction` - The direction to navigate within the carousel (true for next, false for previous).
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
    ///     let response = sender.update_carousel(&carousel, true, &chat_id, &message_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    pub async fn update_carousel(&self, carousel: &VoiceflowCarousel, direction: bool, chat_id: &String, message_id: &String) -> VoiceflousionResult<TelegramResponder> {
        let timestamp = Utc::now().timestamp();

        let api_url = self.prepare_api_url(carousel.has_images(), "edit");

        let (card, index) = carousel.get_next_card(direction)?;

        let body = TelegramSerializer::build_carousel_update_card_body(chat_id, message_id, card, carousel.has_images(), index, carousel.len());

        let telegram_response = self.send_message(&api_url, body).await?;

        if telegram_response.status().is_success() {
            carousel.set_selected_card(index, timestamp);
            TelegramResponder::from_response(telegram_response, VoiceflowBlock::Card(card.clone())).await
        } else {
            let error_text = telegram_response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender update_carousel".to_string(), error_text))
        }
    }

    /// Prepares the appropriate API URL for sending a message based on its type.
    ///
    /// # Parameters
    ///
    /// * `has_image` - Whether the message includes an image.
    /// * `action` - The type of action to perform (e.g., "sendMessage", "sendPhoto", "editMessageText", "editMessageMedia").
    ///
    /// # Returns
    ///
    /// A `String` containing the full API URL.
    fn prepare_api_url(&self, has_image: bool, action: &str) -> String {
        let action_type = if has_image {
            match action {
                "edit" => "editMessageMedia",
                _ => "sendPhoto",
            }
        } else {
            match action {
                "edit" => "editMessageText",
                _ => "sendMessage",
            }
        };
        format!("{}{}/{}", TelegramSender::TELEGRAM_API_URL, self.api_key(), action_type)
    }
}

impl Deref for TelegramSender {
    type Target = SenderBase;

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
    /// * `_client_id` - The client ID (not used in this implementation).
    /// * `text` - The text message to send.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `TelegramResponder` if the request succeeds,
    /// or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::traits::Sender;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::from("chat_id");
    ///     let client_id = String::from("client_id");
    ///     let text = VoiceflowText::new("Hello, World!".to_string());
    ///     let response = sender.send_text(&client_id, text, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_text(&self, _client_id: &String, text: VoiceflowText, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = self.prepare_api_url(false, "send");

        let body = TelegramSerializer::build_text_body(chat_id, text.message());

        let telegram_response = self.send_message(&api_url, body).await?;

        if telegram_response.status().is_success() {
            Self::SenderResponder::from_response(telegram_response, VoiceflowBlock::Text(text)).await
        } else {
            let error_text = telegram_response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_text".to_string(), error_text))
        }
    }

    /// Sends an image message to a chat.
    ///
    /// # Parameters
    ///
    /// * `_client_id` - The client ID (not used in this implementation).
    /// * `image` - The image message to send.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `TelegramResponder` if the request succeeds,
    /// or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowImage;
    /// use voiceflousion::core::traits::Sender;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::from("chat_id");
    ///     let client_id = String::from("client_id");
    ///     let image = VoiceflowImage::new("https://example.com/image.jpg".to_string(), Some(100), Some(200));
    ///     let response = sender.send_image(&client_id, image, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_image(&self, _client_id: &String, image: VoiceflowImage, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = self.prepare_api_url(true, "send");

        let body = TelegramSerializer::build_image_body(chat_id, image.url());

        let telegram_response = self.send_message(&api_url, body).await?;

        if telegram_response.status().is_success() {
            Self::SenderResponder::from_response(telegram_response, VoiceflowBlock::Image(image)).await
        } else {
            let error_text = telegram_response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_image".to_string(), error_text))
        }
    }

    /// Sends a buttons message to a chat.
    ///
    /// # Parameters
    ///
    /// * `_client_id` - The client ID (not used in this implementation).
    /// * `buttons` - The buttons message to send.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `TelegramResponder` if the request succeeds,
    /// or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramSender;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowButton, VoiceflowButtons};
    /// use voiceflousion::core::traits::Sender;
    /// use serde_json::Value;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::from("chat_id");
    ///     let client_id = String::from("client_id");
    ///     let buttons = vec![VoiceflowButton::new("Click me".to_string(), Value::Null, None)];
    ///     let voiceflow_buttons = VoiceflowButtons::new(buttons);
    ///     let response = sender.send_buttons(&client_id, voiceflow_buttons, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_buttons(&self, _client_id: &String, buttons: VoiceflowButtons, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = self.prepare_api_url(false, "send");

        let body = TelegramSerializer::build_buttons_body(chat_id, &buttons);

        let telegram_response = self.send_message(&api_url, body).await?;

        if telegram_response.status().is_success() {
            Self::SenderResponder::from_response(telegram_response, VoiceflowBlock::Buttons(buttons)).await
        } else {
            let error_text = telegram_response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_buttons".to_string(), error_text))
        }
    }


    /// Sends a card message to a chat.
    ///
    /// # Parameters
    ///
    /// * `_client_id` - The client ID (not used in this implementation).
    /// * `card` - The card message to send.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `TelegramResponder` if the request succeeds,
    /// or a `VoiceflousionError` if the request fails.
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
    /// async fn main() {
    ///     let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::from("chat_id");
    ///     let client_id = String::from("client_id");
    ///     let response = sender.send_card(&client_id, card, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_card(&self, _client_id: &String, card: VoiceflowCard, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = self.prepare_api_url(card.image_url().is_some(), "send");

        let body = TelegramSerializer::build_card_body(chat_id, &card);

        let telegram_response = self.send_message(&api_url, body).await?;

        if telegram_response.status().is_success() {
            Self::SenderResponder::from_response(telegram_response, VoiceflowBlock::Card(card)).await
        } else {
            let error_text = telegram_response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_card".to_string(), error_text))
        }
    }

    /// Sends a carousel message to a chat.
    ///
    /// # Parameters
    ///
    /// * `_client_id` - The client ID (not used in this implementation).
    /// * `carousel` - The carousel message to send.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `TelegramResponder` if the request succeeds,
    /// or a `VoiceflousionError` if the request fails.
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
    /// async fn main() {
    ///     let cards = vec![VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None)];
    ///     let carousel = VoiceflowCarousel::new(cards, true);
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::from("chat_id");
    ///     let client_id = String::from("client_id");
    ///     let response = sender.send_carousel(&client_id, carousel, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_carousel(&self, _client_id: &String, carousel: VoiceflowCarousel, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = self.prepare_api_url(carousel.has_images(), "send");

        let (card, index) = carousel.get_selected_card()?;

        let body = TelegramSerializer::build_carousel_card_body(chat_id, card, index, carousel.len());

        let telegram_response = self.send_message(&api_url, body).await?;

        if telegram_response.status().is_success() {
            Self::SenderResponder::from_response(telegram_response, VoiceflowBlock::Carousel(carousel)).await
        } else {
            let error_text = telegram_response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("TelegramSender send_carousel".to_string(), error_text))
        }
    }
}