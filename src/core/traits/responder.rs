use async_trait::async_trait;
use reqwest::Response;
use crate::core::subtypes::SentMessage;
use crate::core::voiceflow::{VoiceflousionError, VoiceflowBlock};

/// A trait that defines the behavior for processing and responding to messages.
///
/// The `Responder` trait provides methods for extracting details from responses,
/// creating `SentMessage` instances, and handling errors.
#[async_trait]
pub trait Responder: Sized + Send + Sync {
    /// Returns a reference to the message ID.
    ///
    /// # Returns
    ///
    /// A reference to the message ID string.
    ///
    /// # Example
    ///
    /// ```
    /// struct MyResponder {
    ///     message_id: String,
    ///     message_content: VoiceflowBlock,
    ///     date: i64,
    /// }
    ///
    /// impl Responder for MyResponder {
    ///     fn message_id(&self) -> &String {
    ///         &self.message_id
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    fn message_id(&self) -> &String;

    /// Returns a reference to the content of the message.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowBlock` representing the message content.
    ///
    /// # Example
    ///
    /// ```
    /// struct MyResponder {
    ///     message_id: String,
    ///     message_content: VoiceflowBlock,
    ///     date: i64,
    /// }
    ///
    /// impl Responder for MyResponder {
    ///     fn message_content(&self) -> &VoiceflowBlock {
    ///         &self.message_content
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    fn message_content(&self) -> &VoiceflowBlock;

    /// Returns the date of the message.
    ///
    /// # Returns
    ///
    /// The date of the message as an `i64` timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// struct MyResponder {
    ///     message_id: String,
    ///     message_content: VoiceflowBlock,
    ///     date: i64,
    /// }
    ///
    /// impl Responder for MyResponder {
    ///     fn date(&self) -> i64 {
    ///         self.date
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    fn date(&self) -> i64;

    /// Creates an instance of the `Responder` from a HTTP response.
    ///
    /// This method processes the HTTP response to extract relevant details and
    /// create an instance of the `Responder`.
    ///
    /// # Parameters
    ///
    /// * `response` - The HTTP response to process.
    /// * `content` - The content of the message as a `VoiceflowBlock`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Responder` instance or a `VoiceflousionError` if the process fails.
    ///
    /// # Example
    ///
    /// ```
    /// use reqwest::Response;
    /// use crate::voiceflow::{VoiceflowBlock, VoiceflousionError};
    /// use async_trait::async_trait;
    ///
    /// struct MyResponder {
    ///     message_id: String,
    ///     message_content: VoiceflowBlock,
    ///     date: i64,
    /// }
    ///
    /// impl Responder for MyResponder {
    ///     fn from_response(response: Response, content: VoiceflowBlock) -> Result<Self, VoiceflousionError> {
    ///         // Conversion implementation
    ///         Ok(MyResponder {
    ///             message_id: String::new(), // Extracted from the response
    ///             message_content: content,
    ///             date: 0, // Extracted from the response
    ///         })
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
    async fn from_response(response: Response, content: VoiceflowBlock) -> Result<Self, VoiceflousionError>;

    /// Creates a `SentMessage` instance from the current message content.
    ///
    /// This method generates a `SentMessage` using the content, message ID, and date
    /// of the current message.
    ///
    /// # Returns
    ///
    /// A `SentMessage` instance containing the details of the current message.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::integrations::core::subtypes::SentMessage;
    ///
    /// let responder = MyResponder {
    ///     message_id: String::from("12345"),
    ///     message_content: VoiceflowBlock::new(), // example initialization
    ///     date: 1624478392,
    /// };
    /// let sent_message = responder.create_sent_message();
    /// ```
    fn create_sent_message(&self) -> SentMessage {
        SentMessage::new(self.message_content().clone(), self.message_id().clone(), self.date())
    }
}
