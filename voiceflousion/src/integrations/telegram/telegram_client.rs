use std::ops::Deref;

use async_trait::async_trait;

use crate::core::base_structs::ClientBase;
use crate::core::ClientBuilder;
use crate::core::session_wrappers::LockedSession;
use crate::core::traits::{Client, Sender};
use crate::core::voiceflow::{State, VoiceflowBlock};
use crate::core::voiceflow::dialog_blocks::VoiceflowCarousel;
use crate::errors::VoiceflousionResult;
use crate::integrations::telegram::{TelegramResponder, TelegramSender, TelegramUpdate};

/// Represents a client for Telegram integration with Voiceflow.
///
/// `TelegramClient` manages the sessions and interactions with the Voiceflow API and Telegram.
pub struct TelegramClient {
    /// The base structure that provides core functionalities.
    client_base: ClientBase<TelegramSender>
}

impl TelegramClient {
    /// Creates a new Telegram client.
    ///
    /// This method initializes a new `TelegramClient` using the provided `ClientBuilder`.
    /// It configures the client with the necessary parameters and returns an instance of `TelegramClient`.
    ///
    /// # Parameters
    ///
    /// * `builder` - The client builder containing necessary configurations.
    ///
    /// # Returns
    ///
    /// A new instance of `TelegramClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = TelegramClient::new(builder);
    /// ```
    pub fn new(builder: ClientBuilder) -> Self{
        let api_key = builder.api_key().clone();
        let max_connections_per_moment = builder.max_connections_per_moment();
        let connection_duration = builder.connection_duration();
        let sender = TelegramSender::new(max_connections_per_moment, api_key, connection_duration);

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
    /// * `message_id` - The ID of the message.
    /// * `index` - The index of the carousel card.
    /// * `interaction_time` - The interaction time.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `H::SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn switch_carousel_card(&self, locked_session: &LockedSession<'_>,  carousel: &VoiceflowCarousel,  message_id: &String, index: usize, interaction_time: i64) -> VoiceflousionResult<TelegramResponder> {
        locked_session.set_last_interaction(Some(interaction_time));
        self.client_base.sender().update_carousel(carousel, index, locked_session.get_chat_id(), message_id).await
    }
}

#[async_trait]
impl Client for TelegramClient{
    type ClientUpdate<'async_trait> = TelegramUpdate;
    type ClientSender<'async_trait> = TelegramSender;

    /// Returns a reference to the `ClientBase`.
    ///
    /// # Returns
    ///
    /// A reference to the `ClientBase` instance.
    fn client_base(&self) -> &ClientBase<Self::ClientSender<'_>> {
        &self.client_base
    }

    /// Handles button interaction by checking if the previous message is a carousel and switching cards if necessary.
    /// Otherwise, processes the button press normally.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The interaction time.
    /// * `message` - The text message associated with the button.
    /// * `button_path` - The data associated with the button.
    /// * `update_state` - The optional state for updating the dialog.
    /// * `update` - The update from the Telegram client.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `H::SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn handle_button_interaction(&self, locked_session: &LockedSession<'_>, interaction_time: i64, message: &String, button_path: &String, update_state: Option<State>, update: &Self::ClientUpdate<'_>, ) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        if let Some(prev_message) = locked_session.previous_message().await.deref() {
            if let VoiceflowBlock::Carousel(carousel) = prev_message.block() {
                if let Some(index) = update.carousel_card_index() {
                    return Ok(vec![self.switch_carousel_card(locked_session, carousel, prev_message.id(), index, interaction_time).await?]);
                }
            }
        }
        self.choose_button_in_voiceflow_dialog(locked_session, interaction_time, message, button_path, update_state).await
    }
}
