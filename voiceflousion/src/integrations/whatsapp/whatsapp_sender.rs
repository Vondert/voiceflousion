use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::SenderBase;
use crate::core::subtypes::HttpClient;
use crate::core::traits::Sender;
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::errors::VoiceflousionResult;
use crate::integrations::whatsapp::whatsapp_responder::WhatsAppResponder;

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

impl WhatsAppSender{
    pub fn new(max_sessions_per_moment: usize, api_key: String, connection_duration: Option<u64>) -> Self {
        Self {
            sender_base: SenderBase::new(max_sessions_per_moment, api_key, connection_duration)
        }
    }
}

#[async_trait]
impl Sender for WhatsAppSender{
    type SenderResponder = WhatsAppResponder;

    async fn send_text(&self, text: VoiceflowText, chat_id: &String, sender_base: &HttpClient, api_key: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_image(&self, image: VoiceflowImage, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_buttons(&self, buttons: VoiceflowButtons, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_card(&self, card: VoiceflowCard, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_carousel(&self, carousel: VoiceflowCarousel, chat_id: &String, sender_http_client: &HttpClient, api_key: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }
}