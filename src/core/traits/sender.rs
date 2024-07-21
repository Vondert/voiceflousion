use async_trait::async_trait;
use crate::core::subtypes::HttpClient;
use crate::core::traits::Responder;
use crate::core::voiceflow::{VoiceflousionError, VoiceflowBlock, VoiceflowMessage};
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};

/// A trait that defines the functionality for sending messages to a client.
///
/// The `Sender` trait provides methods for sending different types of messages,
/// such as text, images, buttons, cards, and carousels, to a client. It also
/// includes a method for sending a complete `VoiceflowMessage`, which can contain
/// multiple types of blocks.
#[async_trait]
pub trait Sender: Send + Sync {
    /// The type that represents the response from the sender.
    type SenderResponder: Responder;

    /// Returns a reference to the HTTP client used for sending requests.
    ///
    /// # Returns
    ///
    /// A reference to the `HttpClient`.
    ///
    /// # Example
    ///
    /// ```
    /// struct MySender {
    ///     http_client: HttpClient,
    ///     api_key: String,
    /// }
    ///
    /// impl Sender for MySender {
    ///     type SenderResponder = MyResponder;
    ///
    ///     fn http_client(&self) -> &HttpClient {
    ///         &self.http_client
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    fn http_client(&self) -> &HttpClient;

    /// Returns a reference to the API key used for authentication.
    ///
    /// # Returns
    ///
    /// A reference to the API key string.
    ///
    /// # Example
    ///
    /// ```
    /// struct MySender {
    ///     http_client: HttpClient,
    ///     api_key: String,
    /// }
    ///
    /// impl Sender for MySender {
    ///     type SenderResponder = MyResponder;
    ///
    ///     fn api_key(&self) -> &String {
    ///         &self.api_key
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    fn api_key(&self) -> &String;

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
    /// A `Result` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = sender.send_message(&chat_id, message).await?;
    /// ```
    async fn send_message(&self, chat_id: &String, message: VoiceflowMessage) -> Result<Vec<Self::SenderResponder>, VoiceflousionError> {
        // Obtain the HTTP client and API key
        let sender_http_client = self.http_client();
        let api_key = self.api_key();
        // Initialize a vector to store responses
        let mut responses = Vec::with_capacity(message.len());

        // Iterate over each block in the message and send it using the appropriate method
        for block in message.into_iter() {
            match block {
                VoiceflowBlock::Text(text) => {
                    let result = self.send_text(text, chat_id, sender_http_client, api_key).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Image(image) => {
                    let result = self.send_image(image, chat_id, sender_http_client, api_key).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Buttons(buttons) => {
                    let result = self.send_buttons(buttons, chat_id, sender_http_client, api_key).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Card(card) => {
                    let result = self.send_card(card, chat_id, sender_http_client, api_key).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Carousel(carousel) => {
                    if !carousel.is_empty() {
                        let result = self.send_carousel(carousel, chat_id, sender_http_client, api_key).await?;
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
    /// A `Result` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// struct MySender {
    ///     http_client: HttpClient,
    ///     api_key: String,
    /// }
    ///
    /// #[async_trait]
    /// impl Sender for MySender {
    ///     type SenderResponder = MyResponder;
    ///
    ///     async fn send_text(&self, text: VoiceflowText, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
    ///         // Implement the sending logic here.
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    async fn send_text(&self, text: VoiceflowText, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError>;

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
    /// A `Result` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// struct MySender {
    ///     http_client: HttpClient,
    ///     api_key: String,
    /// }
    ///
    /// #[async_trait]
    /// impl Sender for MySender {
    ///     type SenderResponder = MyResponder;
    ///
    ///     async fn send_image(&self, image: VoiceflowImage, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
    ///         // Implement the sending logic here.
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    async fn send_image(&self, image: VoiceflowImage, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError>;

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
    /// A `Result` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// struct MySender {
    ///     http_client: HttpClient,
    ///     api_key: String,
    /// }
    ///
    /// #[async_trait]
    /// impl Sender for MySender {
    ///     type SenderResponder = MyResponder;
    ///
    ///     async fn send_buttons(&self, buttons: VoiceflowButtons, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
    ///         // Implement the sending logic here.
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    async fn send_buttons(&self, buttons: VoiceflowButtons, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError>;

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
    /// A `Result` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// struct MySender {
    ///     http_client: HttpClient,
    ///     api_key: String,
    /// }
    ///
    /// #[async_trait]
    /// impl Sender for MySender {
    ///     type SenderResponder = MyResponder;
    ///
    ///     async fn send_card(&self, card: VoiceflowCard, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
    ///         // Implement the sending logic here.
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    async fn send_card(&self, card: VoiceflowCard, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError>;

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
    /// A `Result` containing a `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// struct MySender {
    ///     http_client: HttpClient,
    ///     api_key: String,
    /// }
    ///
    /// #[async_trait]
    /// impl Sender for MySender {
    ///     type SenderResponder = MyResponder;
    ///
    ///     async fn send_carousel(&self, carousel: VoiceflowCarousel, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError> {
    ///         // Implement the sending logic here.
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    async fn send_carousel(&self, carousel: VoiceflowCarousel, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> Result<Self::SenderResponder, VoiceflousionError>;
}
