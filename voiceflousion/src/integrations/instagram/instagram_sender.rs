use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::SenderBase;
use crate::core::traits::Sender;
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::errors::VoiceflousionResult;
use crate::integrations::instagram::InstagramResponder;

pub struct InstagramSender{
    sender_base: SenderBase
}

impl InstagramSender{
    pub fn new(max_sessions_per_moment: usize, api_key: String, connection_duration: Option<u64>) -> Self {
        Self {
            sender_base: SenderBase::new(max_sessions_per_moment, api_key, connection_duration)
        }
    }
}

impl Deref for InstagramSender{
    type Target = SenderBase;

    fn deref(&self) -> &Self::Target {
        &self.sender_base
    }
}

#[async_trait]
impl Sender for InstagramSender{
    type SenderResponder = InstagramResponder;

    async fn send_text(&self, client_id: &String, text: VoiceflowText, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_image(&self, client_id: &String, image: VoiceflowImage, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_buttons(&self, client_id: &String, buttons: VoiceflowButtons, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_card(&self, client_id: &String, card: VoiceflowCard, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }

    async fn send_carousel(&self, client_id: &String, carousel: VoiceflowCarousel, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        todo!()
    }
}