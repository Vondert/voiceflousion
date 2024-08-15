use std::ops::Deref;
use async_trait::async_trait;
use chrono::Utc;
use serde_json::{json, Value};
use crate::core::base_structs::SenderBase;
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

        // Get the current timestamp to record when the card is selected
        let timestamp = Utc::now().timestamp();

        // Retrieve the next card in the carousel based on the direction
        let (card, index) = carousel.get_next_card(direction)?;

        // Convert the buttons from the card to the inline keyboard format
        let inline_keyboard: Vec<Vec<Value>> = carousel_card_buttons_to_keyboard(card, index, carousel.len());

        // Extract the title and description from the card
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        // Initialize the API URL for editing the message
        let mut api_url = format!("{}{}/editMessageMedia", TelegramSender::TELEGRAM_API_URL, self.api_key());

        // If the card has an image URL, create the JSON body for editing the media message
        let body = if carousel.has_images() {
            json!({
                "chat_id": chat_id,
                "message_id": message_id,
                "media": {
                    "type": "photo",
                    "media": card.image_url().as_ref().unwrap().clone(),
                    "caption": format!("{}\n\n{}", title, description),
                },
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            })
        }
        // If there is no image URL, update the API URL and create the JSON body for editing the text message
        else{
            api_url = format!("{}{}/editMessageText", TelegramSender::TELEGRAM_API_URL, self.api_key());
            json!({
                "chat_id": chat_id,
                "message_id": message_id,
                "text": format!("{}\n\n{}", title, description),
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            })
        };

        // Send the POST request with the body to the Telegram API
        let response = self.http_client().post(&api_url).json(&body).send()
            .await.map_err(|e| VoiceflousionError::ClientRequestError("TelegramSender update_carousel".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // If the response is successful, update the carousel's selected card
            // with the new index and timestamp to reflect the change
            carousel.set_selected_card(index, timestamp);

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
    ///     use serde_json::Value;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     let sender = TelegramSender::new(10, "api_key".to_string(), None);
    ///     let chat_id = String::new();
    ///     let client_id = String::new();
    ///     let buttons = vec![VoiceflowButton::new("Click me".to_string(), VoiceflowButtonActionType::Path, Value::Null)];
    ///     let voiceflow_buttons = VoiceflowButtons::new(buttons);
    ///     let response = sender.send_buttons(&client_id, voiceflow_buttons, &chat_id).await;
    ///     println!("{:?}", response);
    /// }
    /// ```
    async fn send_buttons(&self, _client_id: &String, buttons: VoiceflowButtons, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        // Determine the API URL based on the button option (text or image)
        let api_url = format!("{}{}/sendMessage", TelegramSender::TELEGRAM_API_URL, self.api_key());

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
            VoiceflowButtonsOption::Empty => panic!("Buttons with empty text field caught!"),
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

        // Initialize the API URL and request body for sending a card
        let mut api_url = format!("{}{}/sendPhoto", TelegramSender::TELEGRAM_API_URL, self.api_key());

        // Extract the title and description from the card
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        // Convert the buttons from the card to the inline keyboard format
        let inline_keyboard: Vec<Vec<Value>> = card.buttons().as_ref()
            .map(|b| buttons_to_keyboard(b))
            .unwrap_or_else(|| vec![]);

        // If the card has an image URL, update the API URL and request body for sending a photo
        let body = if let Some(url) = card.image_url() {
            json!({
                "chat_id": chat_id,
                "photo": url,
                "caption": format!("{}\n\n{}", title, description),
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            })
        }
        // If there is no image URL, create the JSON body for sending the text message
        else{
            api_url = format!("{}{}/sendMessage", TelegramSender::TELEGRAM_API_URL, self.api_key());
            json!({
                "chat_id": chat_id,
                "text": format!("{}\n\n{}", title, description),
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            })
        };

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
    ///     let cards = vec![VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None)];
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

        // Form the API URL for sending the carousel via Telegram API
        let mut api_url = format!("{}{}/sendPhoto", TelegramSender::TELEGRAM_API_URL, self.api_key());

        // Get the first card and index from the carousel
        let (card, index) = carousel.get_selected_card()?;

        // Extract the title and description from the card
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        // Convert the buttons from the card to the inline keyboard format
        let inline_keyboard: Vec<Vec<Value>> = carousel_card_buttons_to_keyboard(card, index, carousel.len());

        // If the card has an image URL, update the API URL and request body for sending a photo
        let body = if carousel.has_images() {
            json!({
                "chat_id": chat_id,
                "photo": card.image_url().as_ref().unwrap().clone(),
                "caption": format!("{}\n\n{}", title, description),
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            })
        }
        // If there is no image URL, create the JSON body for sending the text message
        else{
            api_url = format!("{}{}/sendMessage", TelegramSender::TELEGRAM_API_URL, self.api_key());
            json!({
                "chat_id": chat_id,
                "text": format!("{}\n\n{}", title, description),
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            })
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
fn buttons_to_keyboard(buttons: &VoiceflowButtons) -> Vec<Vec<Value>> {
    buttons.iter().enumerate().map(|(index, b)| {
        let callback_data = json!({ "index": index }).to_string();
        match &b.action_type() {
            VoiceflowButtonActionType::OpenUrl(url) => {
                let url = if url.is_empty() {
                    // Use "empty" for buttons with no URL specified
                    "empty"
                } else {
                    url
                };
                json!({ "text": b.name(), "url": url, "callback_data": callback_data })
            },
            VoiceflowButtonActionType::Path => json!({ "text": b.name(), "callback_data": callback_data }),
        }
    }).map(|key| vec![key]).collect()
}

/// Converts the buttons of a `VoiceflowCard` into a Telegram-compatible inline keyboard,
/// adding navigation buttons for carousel movement.
///
/// This function generates an inline keyboard for a given `VoiceflowCard`, converting its
/// buttons into a format suitable for use in Telegram's API. Additionally, it appends
/// navigation buttons ("<--" and "-->") to allow users to move through a carousel of cards.
/// The navigation buttons are conditionally added based on the card's position in the carousel
/// and the total number of cards.
///
/// # Parameters
///
/// * `card` - A reference to the `VoiceflowCard` whose buttons will be converted.
/// * `index` - The current position of the card within the carousel (0-based index).
/// * `carousel_len` - The total number of cards in the carousel.
///
/// # Returns
///
/// A `Vec<Vec<Value>>` representing the inline keyboard structure for Telegram,
/// including both the card's buttons and any applicable navigation buttons.
fn carousel_card_buttons_to_keyboard(card: &VoiceflowCard, index: usize, carousel_len: usize) -> Vec<Vec<Value>>{
    let mut inline_keyboard: Vec<Vec<Value>> = card.buttons().as_ref()
        .map(|b| buttons_to_keyboard(b))
        .unwrap_or_else(|| vec![]);

    // Add navigation buttons for the carousel if there are multiple cards
    let mut switch_buttons: Vec<Value> = Vec::new();
    if index > 0 {
        let carousel_prev = json!({
                "direction": format!("{}", false)
            });
        switch_buttons.push(json!({ "text": "<--", "callback_data":  carousel_prev.to_string()}));
    }
    if index < carousel_len - 1 {
        let carousel_next = json!({
                "direction": format!("{}", true)
            });
        switch_buttons.push(json!({ "text": "-->", "callback_data": carousel_next.to_string() }));
    }
    inline_keyboard.push(switch_buttons);

    inline_keyboard
}