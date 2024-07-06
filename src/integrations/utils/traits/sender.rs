use async_trait::async_trait;
use crate::integrations::utils::SenderHttpClient;
use crate::integrations::utils::traits::Message;
use crate::voiceflow::VoiceflousionError;

#[async_trait]
pub trait Sender<M: Message>{
    fn sender_http_client(&self) -> &SenderHttpClient;
    async fn send(&self, message: M) -> Result<(), VoiceflousionError>;
}