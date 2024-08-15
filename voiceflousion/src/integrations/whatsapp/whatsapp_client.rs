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

    async fn handle_carousel_switch(&self, locked_session: &LockedSession<'_>, interaction_time: i64, direction: bool) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        panic!()
    }
}