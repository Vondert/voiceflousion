use std::fmt::Debug;
use std::ops::Deref;
use async_trait::async_trait;
use reqwest::Response;
use crate::core::base_structs::ResponderBase;
use crate::core::subtypes::SentMessage;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::VoiceflousionResult;

/// A trait that defines the behavior for processing and responding to messages.
///
/// The `Responder` trait provides methods for extracting details from responses,
/// creating `SentMessage` instances, and handling errors.
#[async_trait]
pub trait Responder: Deref<Target=ResponderBase> + Sized + Send + Sync + Debug{

    /// Creates an instance of the `Responder` from HTTP response.
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
    /// A `VoiceflousionResult` containing the `Responder` instance or a `VoiceflousionError` if the process fails.
    async fn from_response(response: Response, content: VoiceflowBlock) -> VoiceflousionResult<Self>;

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
