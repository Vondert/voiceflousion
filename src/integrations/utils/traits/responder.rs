use async_trait::async_trait;
use reqwest::Response;
use crate::voiceflow::{VoiceflousionError, VoiceflowBlock};

#[async_trait]
pub trait Responder: Sized + Send + Sync {
    fn message_id(&self) -> &String;
    fn message_content(&self) -> &VoiceflowBlock;
    async fn from_response(response: Response, content: VoiceflowBlock) -> Result<Self, VoiceflousionError>;
}