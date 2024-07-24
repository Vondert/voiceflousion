use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::traits::{ClientBase, Client, Sender};
use crate::integrations::telegram::{TelegramResponder, TelegramSender, TelegramUpdate};
use crate::core::{ClientBuilder, SessionsManager};
use crate::core::session_wrappers::LockedSession;
use crate::core::voiceflow::{State, VoiceflousionError, VoiceflowBlock, VoiceflowClient};
use crate::core::voiceflow::dialog_blocks::VoiceflowCarousel;

/// Represents a client for Telegram integration with Voiceflow.
///
/// `TelegramClient` manages the sessions and interactions with the Voiceflow API and Telegram.
pub struct TelegramClient {
    /// The client ID for the Telegram client.
    client_id: String,
    /// The Voiceflow client for API interactions.
    voiceflow_client: Arc<VoiceflowClient>,
    /// The session manager for handling sessions.
    sessions: Arc<SessionsManager>,
    /// The sender for sending messages via Telegram.
    sender: TelegramSender,
    /// The initial launch state of the client.
    launch_state: State
}
impl TelegramClient{

    /// Creates a new Telegram client.
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
        let client_id = builder.client_id().clone();
        let api_key = builder.api_key().clone();
        let voiceflow_client = builder.voiceflow_client().clone();
        let max_connections_per_moment = builder.max_connections_per_moment();
        let session_duration = builder.session_duration();
        let connection_duration = builder.connection_duration();
        let sessions_cleanup_interval = builder.sessions_cleanup_interval();
        let launch_state = builder.launch_state().clone();
        let sessions= builder.sessions();

        Self{
            client_id,
            voiceflow_client,
            sessions: Arc::new(SessionsManager::new(sessions, session_duration, sessions_cleanup_interval)),
            sender: TelegramSender::new(max_connections_per_moment, api_key, connection_duration),
            launch_state
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
    /// A `Result` containing a `TelegramResponder` or a `VoiceflousionError` if the request fails.
    async fn switch_carousel_card(&self, locked_session: &LockedSession<'_>,  carousel: &VoiceflowCarousel,  message_id: &String, index: usize, interaction_time: i64) -> Result<TelegramResponder, VoiceflousionError> {
        locked_session.set_last_interaction(Some(interaction_time));
        self.sender.update_carousel(carousel, index, locked_session.get_chat_id(), message_id).await
    }
}
impl ClientBase for TelegramClient {
    type ClientUpdate = TelegramUpdate;
    type ClientSender = TelegramSender;

    /// Returns a reference to the client ID.
    ///
    /// # Returns
    ///
    /// A reference to the client ID string.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::traits::ClientBase;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = TelegramClient::new(builder);
    /// let client_id = client.client_id();
    /// ```
    fn client_id(&self) -> &String {
        &self.client_id
    }

    /// Returns a reference to the session manager.
    ///
    /// # Returns
    ///
    /// A reference to the session manager wrapped in an `Arc`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::traits::ClientBase;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = TelegramClient::new(builder);
    /// let sessions = client.sessions();
    /// ```
    fn sessions(&self) -> &Arc<SessionsManager> {
        &self.sessions
    }

    /// Returns a reference to the Voiceflow client.
    ///
    /// # Returns
    ///
    /// A reference to the Voiceflow client wrapped in an `Arc`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::traits::ClientBase;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = TelegramClient::new(builder);
    /// let vf_client = client.voiceflow_client();
    /// ```
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient> {
        &self.voiceflow_client
    }

    /// Returns a reference to the message sender.
    ///
    /// # Returns
    ///
    /// A reference to the message sender.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::traits::ClientBase;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = TelegramClient::new(builder);
    /// let sender = client.sender();
    /// ```
    fn sender(&self) -> &Self::ClientSender {
        &self.sender
    }

    /// Returns a reference to the launch state of the Telegram client.
    ///
    /// This method provides access to the initial state with which the client was configured to start.
    /// The launch state can influence the behavior of the client during its interactions, providing
    /// a base configuration for handling states and responses.
    ///
    /// # Returns
    ///
    /// A reference to the `State` representing the initial conditions or settings for the client's operations.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::traits::ClientBase;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    /// use voiceflousion::core::voiceflow::{VoiceflowClient, State};
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = TelegramClient::new(builder);
    /// let launch_state = client.launch_state();
    /// ```
    fn launch_state(&self) -> &State {
        &self.launch_state
    }
}
#[async_trait]
impl Client for TelegramClient{

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
    /// A `Result` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn handle_button_interaction(&self, locked_session: &LockedSession<'_>, interaction_time: i64, message: &String, button_path: &String, update_state: Option<State>, update: &Self::ClientUpdate, ) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError> {
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
