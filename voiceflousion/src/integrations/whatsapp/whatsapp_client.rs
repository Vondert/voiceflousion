use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::ClientBase;
use crate::core::ClientBuilder;
use crate::core::session_wrappers::LockedSession;
use crate::core::traits::{get_last_sent_message, Client, Sender};
use crate::core::voiceflow::dialog_blocks::VoiceflowCarousel;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::whatsapp::{WhatsAppResponder, WhatsAppUpdate, WhatsAppSender};

/// Represents a client for WhatsApp integration with Voiceflow.
///
/// `WhatsAppClient` manages the sessions and interactions with the Voiceflow API and WhatsApp.
pub struct WhatsAppClient {
    /// The base structure that provides core functionalities.
    client_base: ClientBase<WhatsAppSender>
}
impl WhatsAppClient {

    /// Creates a new WhatsApp client.
    ///
    /// This method initializes a new `WhatsAppClient` using the provided `ClientBuilder`.
    /// It configures the client with the necessary parameters and returns an instance of `WhatsAppClient`.
    ///
    /// # Parameters
    ///
    /// * `builder` - The client builder containing necessary configurations.
    ///
    /// # Returns
    ///
    /// A new instance of `WhatsAppClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::whatsapp::WhatsAppClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = WhatsAppClient::new(builder);
    /// ```
    pub fn new(builder: ClientBuilder) -> Self{
        let api_key = builder.api_key().clone();
        let max_connections_per_moment = builder.max_connections_per_moment();
        let connection_duration = builder.connection_duration();
        let sender = WhatsAppSender::new(max_connections_per_moment, api_key, connection_duration);

        Self{
            client_base: ClientBase::new(builder, sender)
        }
    }

    /// Switches the carousel card at Client's message.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `carousel` - The Voiceflow carousel block.
    /// * `direction` - The direction to switch the carousel (true for next, false for previous).
    /// * `interaction_time` - The time of the interaction.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a `WhatsAppResponder` or a `VoiceflousionError` if the request fails.
    async fn switch_carousel_card(&self, locked_session: &LockedSession<'_>,  carousel: &VoiceflowCarousel, direction: bool, interaction_time: i64) -> VoiceflousionResult<WhatsAppResponder> {
        locked_session.set_last_interaction(Some(interaction_time));
        self.client_base.sender().update_carousel(carousel, direction, self.client_base.client_id(), locked_session.get_chat_id()).await
    }
}

#[async_trait]
impl Client for WhatsAppClient {
    type ClientUpdate<'async_trait> = WhatsAppUpdate;
    type ClientSender<'async_trait> = WhatsAppSender;

    /// Returns a reference to the `ClientBase`.
    ///
    /// # Returns
    ///
    /// A reference to the `ClientBase` instance.
    fn client_base(&self) -> &ClientBase<Self::ClientSender<'_>> {
        &self.client_base
    }

    /// Handles carousel switch interactions in a WhatsApp session.
    ///
    /// This method checks if the previous message contains a carousel block and, if so,
    /// switches the carousel card according to the specified direction. If the previous
    /// message is not a carousel, it returns an error indicating that there is no carousel to switch.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction, ensuring thread-safe access.
    /// * `interaction_time` - The time of the interaction in seconds since the Unix epoch.
    /// * `switch_direction` - The direction to switch the carousel (`true` for next, `false` for previous).
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` if the switch was successful,
    /// or a `VoiceflousionError` if the operation fails (e.g., no carousel to switch).
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