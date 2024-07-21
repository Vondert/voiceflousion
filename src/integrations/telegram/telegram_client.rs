use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::traits::{ClientBase, Client, Update, Sender};
use crate::integrations::telegram::{TelegramResponder, TelegramSender, TelegramUpdate};
use crate::core::{ClientBuilder, SessionsManager};
use crate::core::session_wrappers::LockedSession;
use crate::core::subtypes::InteractionType;
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
    /// let voiceflow_client = Arc::new(VoiceflowClient::new(vf_api_key, bot_id.clone(), version_id, 10, Some(120));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), Arc::new(voiceflow_client), 10);
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
        let sessions= builder.sessions();

        Self{
            client_id,
            voiceflow_client,
            sessions: Arc::new(SessionsManager::new(sessions, session_duration, sessions_cleanup_interval)),
            sender: TelegramSender::new(max_connections_per_moment, api_key, connection_duration)
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
    ///
    /// # Example
    ///
    /// ```
    /// let client = TelegramClient::new(builder);
    /// let message_id = "message_id_value".to_string();
    /// let index = 1;
    /// let interaction_time = 1624478392;
    ///
    /// let response = client.switch_carousel_card(&locked_session, &carousel, &message_id, index, interaction_time).await?;
    /// ```
    pub async fn switch_carousel_card(&self, locked_session: &LockedSession<'_>,  carousel: &VoiceflowCarousel,  message_id: &String, index: usize, interaction_time: i64) -> Result<TelegramResponder, VoiceflousionError> {
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
    /// let client = TelegramClient::new(builder);
    /// let sender = client.sender();
    /// ```
    fn sender(&self) -> &Self::ClientSender {
        &self.sender
    }
}
#[async_trait]
impl Client for TelegramClient{
    /// Interacts with the client based on the provided update.
    ///
    /// This method handles interactions with the Telegram client, updating the dialog state and
    /// sending appropriate responses based on the type of interaction (e.g., button press, text message).
    ///
    /// # Parameters
    ///
    /// * `update` - The update from the Telegram client.
    /// * `launch_state` - The optional state for launching the dialog.
    /// * `update_state` - The optional state for updating the dialog.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let client = TelegramClient::new(builder);
    /// let update = TelegramUpdate::from_request_body(body)?;
    /// let launch_state = None;
    /// let update_state = None;
    ///
    /// let response = client.interact_with_client(update, launch_state, update_state).await?;
    /// ```
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError>{
        let interaction_time =  update.interaction_time();
        if let Some(telegram_session) = self.sessions().get_session(update.chat_id()).await {
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            if let Some(message) = locked_session.previous_message().await.deref(){
                update.is_deprecated(message.date())?
            }
            match update.interaction_type(){
                InteractionType::Button(message, button_path) => {
                    if let Some(message) = locked_session.previous_message().await.deref() {
                        if let VoiceflowBlock::Carousel(carousel) = message.block() {
                            if let Some(index) = update.carousel_card_index() {
                                return Ok(vec![self.switch_carousel_card(&locked_session, carousel, message.id(), index, interaction_time).await?])
                            }
                        }
                    }
                    self.choose_button_in_voiceflow_dialog(&locked_session, interaction_time, message, button_path, update_state).await
                },
                InteractionType::Text(message) | InteractionType::Undefined(message) => {
                    self.send_message_to_voiceflow_dialog(&locked_session, interaction_time, message, update_state).await
                }
            }
        }
        else{
            let telegram_session = self.sessions().add_session(update.chat_id().clone()).await;
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await
        }
    }
}