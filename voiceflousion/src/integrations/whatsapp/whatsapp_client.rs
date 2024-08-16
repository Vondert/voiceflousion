use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::ClientBase;
use crate::core::ClientBuilder;
use crate::core::session_wrappers::LockedSession;
use crate::core::traits::{get_last_sent_message, Client, Sender};
use crate::core::voiceflow::dialog_blocks::VoiceflowCarousel;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::whatsapp::whatsapp_sender::WhatsAppSender;
use crate::integrations::whatsapp::{WhatsAppResponder, WhatsAppUpdate};

pub struct WhatsAppClient {
    client_base: ClientBase<WhatsAppSender>
}
impl WhatsAppClient {
    pub fn new(builder: ClientBuilder) -> Self{
        let api_key = builder.api_key().clone();
        let max_connections_per_moment = builder.max_connections_per_moment();
        let connection_duration = builder.connection_duration();
        let sender = WhatsAppSender::new(max_connections_per_moment, api_key, connection_duration);

        Self{
            client_base: ClientBase::new(builder, sender)
        }
    }

    async fn switch_carousel_card(&self, locked_session: &LockedSession<'_>,  carousel: &VoiceflowCarousel, direction: bool, interaction_time: i64) -> VoiceflousionResult<WhatsAppResponder> {
        locked_session.set_last_interaction(Some(interaction_time));
        self.client_base.sender().update_carousel(carousel, direction, self.client_base.client_id(), locked_session.get_chat_id()).await
    }
}

#[async_trait]
impl Client for WhatsAppClient {
    const ORIGINS: &'static [&'static str] = &[];
    type ClientUpdate<'async_trait> = WhatsAppUpdate;
    type ClientSender<'async_trait> = WhatsAppSender;

    fn client_base(&self) -> &ClientBase<Self::ClientSender<'_>> {
        &self.client_base
    }

    async fn handle_carousel_switch(&self, locked_session: &LockedSession<'_>, interaction_time: i64, switch_direction: bool) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        let response = {
            let binding = locked_session.previous_message().await;
            let previous_message = binding.deref().as_ref()
                .ok_or_else(|| VoiceflousionError::ClientRequestError("WhatsAppClient".to_string(),"Carousel cannot be switched in start of the conversation".to_string()))?;
            if let VoiceflowBlock::Carousel(carousel) = previous_message.block() {

                vec![self.switch_carousel_card(locked_session, carousel, switch_direction, interaction_time).await?]
            }
            else{
                return Err(VoiceflousionError::ValidationError("WhatsAppClient".to_string(),"There is no carousel to switch".to_string()))
            }
        };
        // Retrieve the last message sent by the bot from the response
        let bot_last_message = get_last_sent_message(&response);

        // Update the session with the previous message
        locked_session.set_previous_message(bot_last_message).await;
        Ok(response)
    }
}