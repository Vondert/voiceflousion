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
    async fn switch_carousel_card(&self, locked_session: &LockedSession<'_>,  carousel: &VoiceflowCarousel,  message_id: &String, direction: bool, interaction_time: i64) -> VoiceflousionResult<TelegramResponder> {
        locked_session.set_last_interaction(Some(interaction_time));
        self.client_base.sender().update_carousel(carousel, direction, locked_session.get_chat_id(), message_id).await
    }
}

#[async_trait]
impl Client for TelegramClient{

    /// An array of allowed origins for CORS specific to the Telegram client.
    const ORIGINS: &'static [&'static str] = &[
        "http://149.154.160.0",
        "http://149.154.160.1",
        "http://149.154.160.2",
        "http://149.154.160.3",
        "http://149.154.160.4",
        "http://149.154.160.5",
        "http://149.154.160.6",
        "http://149.154.160.7",
        "http://149.154.160.8",
        "http://149.154.160.9",
        "http://149.154.160.10",
        "http://149.154.160.11",
        "http://149.154.160.12",
        "http://149.154.160.13",
        "http://149.154.160.14",
        "http://149.154.160.15",
        "http://149.154.160.16",
        "http://149.154.160.17",
        "http://149.154.160.18",
        "http://149.154.160.19",
        "http://149.154.160.20",

        "http://91.108.4.0",
        "http://91.108.4.1",
        "http://91.108.4.2",
        "http://91.108.4.3",
        "http://91.108.4.4",
        "http://91.108.4.5",
        "http://91.108.4.6",
        "http://91.108.4.7",
        "http://91.108.4.8",
        "http://91.108.4.9",
        "http://91.108.4.10",
        "http://91.108.4.11",
        "http://91.108.4.12",
        "http://91.108.4.13",
        "http://91.108.4.14",
        "http://91.108.4.15",
        "http://91.108.4.16",
        "http://91.108.4.17",
        "http://91.108.4.18",
        "http://91.108.4.19",
        "http://91.108.4.20",
        "http://91.108.4.21",
        "http://91.108.4.22"
    ];

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


    /// Handles button interactions by checking if the previous message is a carousel and switching cards if necessary.
    /// Otherwise, processes the button press normally.
    ///
    /// This method determines if the interaction involves a carousel and switches the carousel
    /// card accordingly. If not, it handles the button press as usual.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The time of the interaction.
    /// * `update_state` - The optional state for updating the dialog.
    /// * `update` - The update from the Telegram client.
    /// * `button_index` - The index of the button being interacted with.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn handle_button_interaction(&self, locked_session: &LockedSession<'_>, interaction_time: i64, update_state: Option<State>, update: &Self::ClientUpdate<'_>, button_index: i64) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        let payload = if button_index >= 0{
            let button_index = button_index as usize;
            let binding = locked_session.previous_message().await;
            let previous_message = binding.deref().as_ref().expect("No buttons to handle in previous message!");
            previous_message.get_button_payload(button_index)?
        }
        else{
            let binding = locked_session.previous_message().await;
            let previous_message = binding.deref().as_ref().expect("No buttons to handle in previous message!");
            if let VoiceflowBlock::Carousel(carousel) = previous_message.block() {
                if let Some(carousel_direction) = update.carousel_card_index() {
                    return Ok(vec![self.switch_carousel_card(locked_session, carousel, previous_message.id(), carousel_direction, interaction_time).await?]);
                }
            }
            panic!("Invalid direction buttons interaction")
        };
        self.choose_button_in_voiceflow_dialog(locked_session, interaction_time, update_state, payload).await
    }
}
