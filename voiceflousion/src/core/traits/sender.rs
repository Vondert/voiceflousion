use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::SenderBase;
use crate::core::subtypes::HttpClient;
use crate::core::traits::Responder;
use crate::core::voiceflow::{VoiceflowBlock, VoiceflowMessage};
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// A trait that defines the functionality for sending messages to a client.
///
/// The `Sender` trait provides methods for sending different types of messages,
/// such as text, images, buttons, cards, and carousels, to a client. It also
/// includes a method for sending a complete `VoiceflowMessage`, which can contain
/// multiple types of blocks.
#[async_trait]
pub trait Sender: Deref<Target=SenderBase> + Send + Sync + Sized {
    /// The type that represents the response from the sender.
    type SenderResponder: Responder;

    /// Sends a `VoiceflowMessage` to a client.
    ///
    /// **This method has base implementation for sending messages. Modify it only if you
    /// know what you are doing or have devised a better approach.**
    ///
    /// This method iterates over the blocks in the `VoiceflowMessage` and sends
    /// each block using the appropriate method (`send_text`, `send_image`, etc.).
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the client to send the message to.
    /// * `message` - The `VoiceflowMessage` to send.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn send_message(&self, client_id: &String, chat_id: &String, message: VoiceflowMessage) -> VoiceflousionResult<Vec<Self::SenderResponder>> {
        // Obtain the HTTP client and API key
        let sender_http_client = self.http_client();
        let api_key = self.api_key();
        // Initialize a vector to store responses
        let mut responses = Vec::with_capacity(message.len());

        // Iterate over each block in the message and send it using the appropriate method
        for block in message.into_iter() {
            match block {
                VoiceflowBlock::Text(text) => {
                    let result = self.send_text(client_id, text, chat_id).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Image(image) => {
                    let result = self.send_image(client_id, image, chat_id).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Buttons(buttons) => {
                    let result = self.send_buttons(client_id, buttons, chat_id).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Card(card) => {
                    let result = self.send_card(client_id, card, chat_id).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Carousel(carousel) => {
                    if !carousel.is_empty() {
                        let result = self.send_carousel(client_id, carousel, chat_id).await?;
                        responses.push(result)
                    }
                }
                _ => {
                    return Err(VoiceflousionError::ClientRequestInvalidBodyError(
                        "Sender send_message".to_string(),
                        "Unsendable block type in the VoiceflowMessage".to_string(),
                    ))
                },
            }
        }

        Ok(responses)
    }

    /// Sends a text message to a client.
    ///
    /// # Parameters
    ///
    /// * `text` - The `VoiceflowText` block to send.
    /// * `chat_id` - The chat ID of the client to send the message to.
    /// * `sender_http_client` - The HTTP client used for sending the request.
    /// * `api_key` - The API key used for authentication.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn send_text(&self, client_id: &String, text: VoiceflowText, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder>;

    /// Sends an image message to a client.
    ///
    /// # Parameters
    ///
    /// * `image` - The `VoiceflowImage` block to send.
    /// * `chat_id` - The chat ID of the client to send the message to.
    /// * `sender_http_client` - The HTTP client used for sending the request.
    /// * `api_key` - The API key used for authentication.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn send_image(&self, client_id: &String, image: VoiceflowImage, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder>;

    /// Sends a button message to a client.
    ///
    /// # Parameters
    ///
    /// * `buttons` - The `VoiceflowButtons` block to send.
    /// * `chat_id` - The chat ID of the client to send the message to.
    /// * `sender_http_client` - The HTTP client used for sending the request.
    /// * `api_key` - The API key used for authentication.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn send_buttons(&self, client_id: &String, buttons: VoiceflowButtons, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder>;

    /// Sends a card message to a client.
    ///
    /// # Parameters
    ///
    /// * `card` - The `VoiceflowCard` block to send.
    /// * `chat_id` - The chat ID of the client to send the message to.
    /// * `sender_http_client` - The HTTP client used for sending the request.
    /// * `api_key` - The API key used for authentication.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn send_card(&self, client_id: &String, card: VoiceflowCard, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder>;

    /// Sends a carousel message to a client.
    ///
    /// # Parameters
    ///
    /// * `carousel` - The `VoiceflowCarousel` block to send.
    /// * `chat_id` - The chat ID of the client to send the message to.
    /// * `sender_http_client` - The HTTP client used for sending the request.
    /// * `api_key` - The API key used for authentication.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn send_carousel(&self, client_id: &String, carousel: VoiceflowCarousel, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder>;
}
