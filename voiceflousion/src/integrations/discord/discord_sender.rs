use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::SenderBase;
use crate::core::traits::Sender;
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::errors::VoiceflousionResult;
use crate::integrations::discord::discord_responder::DiscordResponder;

pub struct DiscordSender{
    /// The base structure that provides core functionalities.
    sender_base: SenderBase
}

impl Deref for DiscordSender {
    type Target = SenderBase;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl DiscordSender{
    /// Creates a new `DiscordSender`.
    ///
    /// # Parameters
    ///
    /// * `max_sessions_per_moment` - The maximum number of idle connections per host.
    /// * `api_key` - The API key for authenticating with the Discord API.
    /// * `connection_duration` - The optional duration for which sessions can remain idle (in seconds).
    ///
    /// # Returns
    ///
    /// A new instance of `DiscordSender`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::discord::DiscordSender;
    ///
    /// let sender = DiscordSender::new(10, "api_key".to_string(), Some(120));
    /// let default_duration_sender = DiscordSender::new(10, "api_key".to_string(), None);
    /// ```
    pub fn new(max_sessions_per_moment: usize, api_key: String, connection_duration: Option<u64>) -> Self {
        Self {
            sender_base: SenderBase::new(max_sessions_per_moment, api_key, connection_duration)
        }
    }
}

#[async_trait]
impl Sender for DiscordSender{
    type SenderResponder = DiscordResponder;

    async fn send_text(&self, client_id: &String, text: VoiceflowText, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        unimplemented!()
    }

    async fn send_image(&self, client_id: &String, image: VoiceflowImage, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        unimplemented!()
    }

    async fn send_buttons(&self, client_id: &String, buttons: VoiceflowButtons, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        unimplemented!()
    }

    async fn send_card(&self, client_id: &String, card: VoiceflowCard, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        unimplemented!()
    }

    async fn send_carousel(&self, client_id: &String, carousel: VoiceflowCarousel, chat_id: &String) -> VoiceflousionResult<Self::SenderResponder> {
        unimplemented!()
    }
}