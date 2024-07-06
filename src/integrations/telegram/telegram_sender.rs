use async_trait::async_trait;
use reqwest::Client;
use crate::integrations::telegram::TelegramMessage;
use crate::integrations::utils::SenderHttpClient;
use crate::integrations::utils::traits::Sender;
use crate::voiceflow::VoiceflousionError;

static TELEGRAM_API_URL: &str = "";
pub struct TelegramSender{
    sender_http_client: SenderHttpClient
}
impl TelegramSender{
    pub fn new(max_sessions_per_moment: usize) -> Self{
        Self{
            sender_http_client: SenderHttpClient::new(max_sessions_per_moment)
        }
    }
}
#[async_trait]
impl Sender<TelegramMessage> for TelegramSender{
    fn sender_http_client(&self) -> &SenderHttpClient {
        &self.sender_http_client
    }

    async fn send(&self, message: TelegramMessage) -> Result<(), VoiceflousionError> {
        todo!()
    }
}