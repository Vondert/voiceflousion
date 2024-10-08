use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;
use crate::core::base_structs::SessionsManager;
use crate::core::ClientBuilder;
use crate::core::subtypes::BotAuthToken;
use crate::core::traits::Sender;
use crate::core::voiceflow::{State, VoiceflowClient};


/// `ClientBase` is the foundational struct for managing client interactions with Voiceflow.
///
/// This struct encapsulates essential components such as the client ID, Voiceflow client for API interactions,
/// session manager for handling sessions, message sender, the initial launch state, and additional internal states
/// like the status flag and bot authentication token.
pub struct ClientBase<H: Sender> {
    /// The unique identifier for the client.
    client_id: String,
    /// The Voiceflow client used for interacting with the Voiceflow API.
    voiceflow_client: Arc<VoiceflowClient>,
    /// The session manager responsible for managing and maintaining sessions.
    sessions: SessionsManager,
    /// The sender used for sending messages through the appropriate channel.
    sender: H,
    /// The initial launch state that determines the client's starting condition.
    launch_state: State,
    /// A status flag indicating the current operational state of the client.
    status: Arc<AtomicBool>,
    /// The bot authentication token, which may be updated or accessed as needed.
    bot_auth_token: Arc<RwLock<Option<BotAuthToken>>>,
}

impl<H: Sender> ClientBase<H> {
    /// Creates a new instance of `ClientBase`.
    ///
    /// # Parameters
    ///
    /// * `builder` - The client builder containing necessary configurations.
    /// * `sender` - The message sender for client interactions.
    ///
    /// # Returns
    ///
    /// A new instance of `ClientBase`.
    pub fn new(builder: ClientBuilder, sender: H) -> Self{
        let client_id = builder.client_id().clone();
        let voiceflow_client = builder.voiceflow_client().clone();
        let session_duration = builder.session_duration();
        let sessions_cleanup_interval = builder.sessions_cleanup_interval();
        let launch_state = builder.launch_state().clone();
        let status = builder.status();
        let secret_auth_token = builder.bot_auth_token().clone();
        let sessions= builder.sessions();

        Self{
            client_id,
            voiceflow_client,
            sessions: SessionsManager::new(sessions, session_duration, sessions_cleanup_interval),
            sender,
            launch_state,
            status: Arc::new(AtomicBool::new(status)),
            bot_auth_token: Arc::new(RwLock::new(
                if let Some(token) = secret_auth_token{
                    Some(BotAuthToken::new(token))
                }
                else{
                    None
                }

            ))
        }
    }

    /// Returns a reference to the client ID.
    ///
    /// # Returns
    ///
    /// A reference to the client ID string.
    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    /// Returns a reference to the session manager.
    ///
    /// # Returns
    ///
    /// A reference to the session.
    pub fn sessions(&self) -> &SessionsManager {
        &self.sessions
    }

    /// Returns a reference to the Voiceflow client.
    ///
    /// # Returns
    ///
    /// A reference to the Voiceflow client wrapped in an `Arc`.
    pub fn voiceflow_client(&self) -> &Arc<VoiceflowClient> {
        &self.voiceflow_client
    }

    /// Returns a reference to the message sender.
    ///
    /// # Returns
    ///
    /// A reference to the message sender.
    pub fn sender(&self) -> &H {
        &self.sender
    }

    /// Returns a reference to the launch state of the client.
    ///
    /// This method provides access to the initial state with which the client was configured to start.
    /// The launch state can influence the behavior of the client during its interactions, providing
    /// a base configuration for handling states and responses.
    ///
    /// # Returns
    ///
    /// A reference to the `State` representing the initial conditions or settings for the client's operations.
    pub fn launch_state(&self) -> &State {
        &self.launch_state
    }

    /// Checks if the client is active.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the client is active.
    pub fn is_active(&self) -> bool {
        self.status.load(Ordering::SeqCst)
    }

    /// Activates the client and starts the sessions cleanup process.
    pub fn activate(&self) {
        self.status.store(true, Ordering::SeqCst);
        if let Some(interval) = self.sessions.cleanup_interval() {
            self.sessions.start_cleanup(interval);
        }
    }

    /// Deactivates the client and stops the sessions cleanup process.
    pub fn deactivate(&self) {
        self.status.store(false, Ordering::SeqCst);
        self.sessions.stop_cleanup();
    }

    /// Returns the bot authentication token.
    ///
    /// # Returns
    ///
    /// An optional `BotAuthToken`.
    pub async fn bot_auth_token(&self) -> Option<BotAuthToken> {
        let read = self.bot_auth_token.read().await;
        read.deref().clone()
    }

    /// Changes the bot authentication token.
    ///
    /// # Parameters
    ///
    /// * `token` - An optional new token.
    pub async fn change_bot_auth_token(&self, token: Option<String>) {
        let bot_auth_token = token.map(BotAuthToken::new);
        let mut write = self.bot_auth_token.write().await;
        *write = bot_auth_token;
    }

    /// Destructures the client base into a `ClientBuilder` without sessions.
    ///
    /// This method creates a `ClientBuilder` with the client's current configurations,
    /// excluding the sessions. This is useful for reconfiguring or recreating a client
    /// with the same initial settings.
    ///
    /// # Returns
    ///
    /// A `ClientBuilder` instance without sessions.
    pub async fn destructure_to_client_builder_without_sessions(&self) -> ClientBuilder{
        let client_id = self.client_id.clone();
        let api_key = self.sender.api_key().clone();
        let voiceflow_client = self.voiceflow_client.clone();
        let max_connections_per_moment = self.sender.http_client().max_connections_per_moment();
        let connection_duration = self.sender.http_client().connection_duration();
        let launch_state = self.launch_state.clone();
        let status = self.is_active();
        let token = self.bot_auth_token().await.map(|token| token.token().clone());

        let mut builder = ClientBuilder::new(client_id, api_key, voiceflow_client, max_connections_per_moment)
            .set_connection_duration(connection_duration)
            .set_launch_state(launch_state)
            .set_status(status);

        builder = if let Some(interval) =  self.sessions.cleanup_interval(){
            builder.allow_sessions_cleaning(interval)
        }
        else {
            builder
        };

        builder = if let Some(token) = token{
            builder.set_bot_auth_token(token)
        }
        else {
            builder
        };

        if let Some(duration) = self.sessions.valid_session_duration(){
            builder.set_session_duration(duration)
        }
        else {
            builder
        }
    }
}