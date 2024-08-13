use std::ops::Deref;
use async_trait::async_trait;
use serde_json::Value;
use crate::core::base_structs::ClientBase;
use crate::core::ClientBuilder;
use crate::core::session_wrappers::LockedSession;
use crate::core::subtypes::SentMessage;
use crate::core::traits::{Client, Sender};
use crate::core::voiceflow::{State, VoiceflowBlock};
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::whatsapp::whatsapp_sender::WhatsAppSender;
use crate::integrations::whatsapp::WhatsAppUpdate;

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
}

#[async_trait]
impl Client for WhatsAppClient {
    const ORIGINS: &'static [&'static str] = &[];
    type ClientUpdate<'async_trait> = WhatsAppUpdate;
    type ClientSender<'async_trait> = WhatsAppSender;

    fn client_base(&self) -> &ClientBase<Self::ClientSender<'_>> {
        &self.client_base
    }

    async fn handle_button_interaction(&self, locked_session: &LockedSession<'_>, interaction_time: i64, update_state: Option<State>, update: &Self::ClientUpdate<'_>, payload: &Value) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        let is_valid_button_click = is_valid_button_click(locked_session, update).await;
        if is_valid_button_click{
            return self.choose_button_in_voiceflow_dialog(locked_session, interaction_time, update_state, payload).await
        }
        Err(VoiceflousionError::DeprecatedError(update.chat_id().clone(), update.update_id().clone()))
    }
}
async fn is_valid_button_click(locked_session: &LockedSession<'_>, update: &WhatsAppUpdate) -> bool{
    let binding = locked_session.previous_message().await;
    let previous_message: &SentMessage = match binding.deref().as_ref() {
        Some(msg) => msg,
        None => return false,
    };

    match previous_message.block() {
        VoiceflowBlock::Buttons(buttons) => buttons.mark() == update.mark(),
        VoiceflowBlock::Card(card) => card
            .buttons()
            .as_ref()
            .map_or(false, |buttons| buttons.mark() == update.mark()),
        _ => false,
    }
}