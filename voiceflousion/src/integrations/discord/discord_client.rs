use async_trait::async_trait;
use crate::core::base_structs::ClientBase;
use crate::core::ClientBuilder;
use crate::core::session_wrappers::LockedSession;
use crate::core::traits::{Client, Sender};
use crate::errors::VoiceflousionResult;
use crate::integrations::discord::discord_sender::DiscordSender;
use crate::integrations::discord::discord_update::DiscordUpdate;

pub struct DiscordClient{
    client_base: ClientBase<DiscordSender>
}

impl DiscordClient{
    /// Creates a new `DiscordClient`.
    ///
    /// This method initializes a new `TelegramClient` using the provided `ClientBuilder`.
    /// It configures the client with the necessary parameters and returns an instance of `DiscordClient`.
    ///
    /// # Parameters
    ///
    /// * `builder` - The `ClientBuilder` containing the necessary configurations.
    ///
    /// # Returns
    ///
    /// A new instance of `DiscordClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::discord::DiscordClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = DiscordClient::new(builder);
    /// ```
    pub fn new(builder: ClientBuilder) -> Self {
        let api_key = builder.api_key().clone();
        let max_connections_per_moment = builder.max_connections_per_moment();
        let connection_duration = builder.connection_duration();
        let sender = DiscordSender::new(max_connections_per_moment, api_key, connection_duration);

        Self {
            client_base: ClientBase::new(builder, sender),
        }
    }
}

#[async_trait]
impl Client for DiscordClient{
    type ClientUpdate<'async_trait> = DiscordUpdate;
    type ClientSender<'async_trait> = DiscordSender;

    fn client_base(&self) -> &ClientBase<Self::ClientSender<'_>> {
        &self.client_base
    }

    async fn handle_carousel_switch(&self, locked_session: &LockedSession<'_>, interaction_time: i64, switch_direction: bool) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        todo!()
    }
}