use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::ClientBase;
use crate::core::ClientBuilder;
use crate::core::session_wrappers::LockedSession;
use crate::core::traits::{Client, Sender};
use crate::core::voiceflow::State;
use crate::errors::VoiceflousionResult;
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

    async fn handle_button_interaction(&self, locked_session: &LockedSession<'_>, interaction_time: i64, update_state: Option<State>, update: &Self::ClientUpdate<'_>, button_index: i64) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        let payload = {
            let button_index = button_index as usize;
            let binding = locked_session.previous_message().await;
            let previous_message = binding.deref().as_ref().expect("No buttons to handle in previous message!");
            previous_message.get_button_payload(button_index)?
        };
        self.choose_button_in_voiceflow_dialog(locked_session, interaction_time, update_state, payload.clone()).await
    }
}