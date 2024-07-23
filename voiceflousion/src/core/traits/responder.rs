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
    fn message_id(&self) -> &String;

    /// Returns a reference to the content of the message.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowBlock` representing the message content.
    fn message_content(&self) -> &VoiceflowBlock;

    /// Returns the date of the message.
    ///
    /// # Returns
    ///
    /// The date of the message as an `i64` timestamp.
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
    async fn from_response(response: Response, content: VoiceflowBlock) -> Result<Self, VoiceflousionError>;

    /// Creates a `SentMessage` instance from the current message content.
    ///
    /// This method generates a `SentMessage` using the content, message ID, and date
    /// of the current message.
    ///
    /// # Returns
    ///
    /// A `SentMessage` instance containing the details of the current message.
    fn create_sent_message(&self) -> SentMessage {
        SentMessage::new(self.message_content().clone(), self.message_id().clone(), self.date())
    }
}
