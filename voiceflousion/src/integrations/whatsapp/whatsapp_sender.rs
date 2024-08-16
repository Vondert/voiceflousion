use std::ops::Deref;
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Response;
use serde_json::Value;
use crate::core::base_structs::SenderBase;
use crate::core::traits::{Responder, Sender};
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::whatsapp::whatsapp_responder::WhatsAppResponder;
use crate::integrations::whatsapp::whatsapp_serializer::WhatsAppSerializer;


/// Represents a sender for WhatsApp integration.
///
/// `WhatsAppSender` handles sending various types of messages (text, image, buttons, etc.)
/// to a WhatsApp client using the WhatsApp API.
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

impl WhatsAppSender {

    /// The base URL for WhatsApp API.
    const WHATSAPP_API_URL: &'static str = "https://graph.facebook.com/v20.0/";

    /// Creates a new instance of `WhatsAppSender`.
    ///
    /// # Parameters
    ///
    /// * `max_sessions_per_moment` - The maximum number of sessions per moment.
    /// * `api_key` - The API key for authenticating requests.
    /// * `connection_duration` - Optional connection duration.
    ///
    /// # Returns
    ///
    /// A new instance of `WhatsAppSender`.
    pub fn new(max_sessions_per_moment: usize, api_key: String, connection_duration: Option<u64>) -> Self {
        Self {
            sender_base: SenderBase::new(max_sessions_per_moment, api_key, connection_duration)
        }
    }

    /// Sends a message to the WhatsApp API.
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
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .send()
            .await
            .map_err(|e| VoiceflousionError::ClientRequestError("WhatsAppSender send_message".to_string(), e.to_string()))
    }

    /// Sends parts of a card to the WhatsApp API.
    ///
    /// # Parameters
    ///
    /// * `api_url` - The API endpoint URL.
    /// * `card_parts` - A vector of JSON bodies representing the parts of the card.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `Response` or a `VoiceflousionError` if the request fails.
    async fn send_card_parts(&self, api_url: &str, card_parts: Vec<Value>) -> VoiceflousionResult<Response> {
        let mut last_response = None;

        for body in card_parts {
            let response = self.send_message(api_url, body).await?;
            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(VoiceflousionError::ClientRequestError("WhatsAppSender send_card_parts".to_string(), error_text));
            }
            last_response = Some(response);
        }

        Ok(last_response.expect("Empty response"))
    }

    /// Updates a carousel message in a WhatsApp chat.
    ///
    /// # Parameters
    ///
    /// * `carousel` - The carousel to update.
    /// * `direction` - The direction to navigate within the carousel (true for next, false for previous).
    /// * `client_id` - The client ID for the WhatsApp API.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `WhatsAppResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = whatsapp_sender.update_carousel(&carousel, true, &client_id, &chat_id).await?;
    /// ```
    pub async fn update_carousel(&self, carousel: &VoiceflowCarousel, direction: bool, client_id: &String, chat_id: &String) -> VoiceflousionResult<WhatsAppResponder> {
        let api_url = Self::prepare_api_url(client_id);
        let timestamp = Utc::now().timestamp();
        let (card, index) = carousel.get_next_card(direction)?;
        let mut interactive_rows = WhatsAppSerializer::carousel_card_buttons_to_list_rows(card, timestamp, index, carousel.len());

        interactive_rows.truncate(10);

        let card_parts = WhatsAppSerializer::prepare_card_parts(carousel.has_images(), card, &interactive_rows, chat_id);
        let response = self.send_card_parts(&api_url, card_parts).await?;
        carousel.set_selected_card(index, timestamp);
        WhatsAppResponder::from_response(response, VoiceflowBlock::Carousel(carousel.clone())).await
    }

    /// Prepares the API URL for sending messages.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID for the WhatsApp API.
    ///
    /// # Returns
    ///
    /// A `String` representing the full API URL.
    fn prepare_api_url(client_id: &str) -> String {
        format!("{}{}/messages", Self::WHATSAPP_API_URL, client_id)
    }
}

#[async_trait]
impl Sender for WhatsAppSender {
    type SenderResponder = WhatsAppResponder;

    /// Sends a text message via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID for the WhatsApp API.
    /// * `text` - The `VoiceflowText` object containing the message.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `WhatsAppResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = whatsapp_sender.send_text(&client_id, text, &chat_id).await?;
    /// ```
    async fn send_text(&self, client_id: &String, text: VoiceflowText, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = Self::prepare_api_url(client_id);
        let body = WhatsAppSerializer::prepare_text_body(chat_id, text.message());
        let response = self.send_message(&api_url, body).await?;

        if response.status().is_success() {
            Self::SenderResponder::from_response(response, VoiceflowBlock::Text(text)).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("WhatsAppSender send_text".to_string(), error_text))
        }
    }

    /// Sends an image message via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID for the WhatsApp API.
    /// * `image` - The `VoiceflowImage` object containing the image URL.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `WhatsAppResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = whatsapp_sender.send_image(&client_id, image, &chat_id).await?;
    /// ```
    async fn send_image(&self, client_id: &String, image: VoiceflowImage, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = Self::prepare_api_url(client_id);
        let body = WhatsAppSerializer::prepare_image_body(chat_id, image.url());
        let response = self.send_message(&api_url, body).await?;

        if response.status().is_success() {
            Self::SenderResponder::from_response(response, VoiceflowBlock::Image(image)).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("WhatsAppSender send_image".to_string(), error_text))
        }
    }

    /// Sends a buttons message via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID for the WhatsApp API.
    /// * `buttons` - The `VoiceflowButtons` object containing the buttons configuration.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `WhatsAppResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = whatsapp_sender.send_buttons(&client_id, buttons, &chat_id).await?;
    /// ```
    async fn send_buttons(&self, client_id: &String, buttons: VoiceflowButtons, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = Self::prepare_api_url(client_id);
        let interactive_rows = WhatsAppSerializer::buttons_to_list_rows(&buttons, buttons.mark());
        let text = match &buttons.option() {
            VoiceflowButtonsOption::Text(text) => text.message(),
            VoiceflowButtonsOption::Empty => panic!("Buttons with empty text field caught!"),
        };
        let body = WhatsAppSerializer::prepare_interactive_body(chat_id, text, interactive_rows);
        let response = self.send_message(&api_url, body).await?;

        if response.status().is_success() {
            Self::SenderResponder::from_response(response, VoiceflowBlock::Buttons(buttons)).await
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(VoiceflousionError::ClientRequestError("WhatsAppSender send_buttons".to_string(), error_text))
        }
    }

    /// Sends a card message via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID for the WhatsApp API.
    /// * `card` - The `VoiceflowCard` object containing the card details.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `WhatsAppResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = whatsapp_sender.send_card(&client_id, card, &chat_id).await?;
    /// ```
    async fn send_card(&self, client_id: &String, card: VoiceflowCard, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = Self::prepare_api_url(client_id);
        let interactive_rows = if let Some(buttons) = card.buttons() {
            WhatsAppSerializer::buttons_to_list_rows(buttons, buttons.mark())
        } else {
            vec![]
        };

        let card_parts = WhatsAppSerializer::prepare_card_parts(card.image_url().is_some(), &card, &interactive_rows, chat_id);
        let response = self.send_card_parts(&api_url, card_parts).await?;

        Self::SenderResponder::from_response(response, VoiceflowBlock::Card(card)).await
    }

    /// Sends a carousel message via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID for the WhatsApp API.
    /// * `carousel` - The `VoiceflowCarousel` object containing the carousel details.
    /// * `chat_id` - The chat ID of the recipient.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `WhatsAppResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = whatsapp_sender.send_carousel(&client_id, carousel, &chat_id).await?;
    /// ```
    async fn send_carousel(&self, client_id: &String, carousel: VoiceflowCarousel, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        let api_url = Self::prepare_api_url(client_id);
        let (card, index) = carousel.get_selected_card()?;
        let mark = carousel.get_selected_mark();
        let mut interactive_rows = WhatsAppSerializer::carousel_card_buttons_to_list_rows(card, mark, index, carousel.len());
        interactive_rows.truncate(10);
        let card_parts = WhatsAppSerializer::prepare_card_parts(carousel.has_images(), card, &interactive_rows, chat_id);
        let response = self.send_card_parts(&api_url, card_parts).await?;
        WhatsAppResponder::from_response(response, VoiceflowBlock::Carousel(carousel.clone())).await
    }
}