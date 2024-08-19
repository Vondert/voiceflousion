use std::sync::Arc;
use crate::core::session_wrappers::Session;
use crate::core::voiceflow::{State, VoiceflowClient};


/// Builds a client with the necessary configurations.
///
/// `ClientBuilder` provides a builder pattern for constructing a client with various configurations,
/// including session management, cleanup settings, and API interactions. This struct allows for
/// flexible client setup tailored to specific needs.
pub struct ClientBuilder {
    /// The client ID used to uniquely identify the client.
    client_id: String,
    /// The API key for authentication with the Voiceflow API.
    api_key: String,
    /// The Voiceflow client used for API interactions.
    voiceflow_client: Arc<VoiceflowClient>,
    /// Optional sessions to initialize the client with. This can be used to preload existing sessions.
    sessions: Option<Vec<Session>>,
    /// The maximum number of connections allowed at any given moment.
    max_connections_per_moment: usize,
    /// Optional duration of the HTTP connection in seconds. This controls how long connections are kept alive.
    connection_duration: Option<u64>,
    /// The optional duration for session validity in seconds. This defines how long a session remains valid before expiring.
    session_duration: Option<i64>,
    /// The optional interval for session cleanup in seconds. This defines how frequently the system checks for expired sessions and cleans them up.
    sessions_cleanup_interval: Option<u64>,
    /// The launch state of the Voiceflow client, which may include configurations like default states or variables.
    launch_state: State,
    /// A status flag indicating whether the client is active or not.
    status: bool,
    /// The optional bot authentication token, used for additional security or identification purposes.
    bot_auth_token: Option<String>,
}

impl ClientBuilder {
    /// Creates a new `ClientBuilder`.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID.
    /// * `api_key` - The API key for authentication.
    /// * `voiceflow_client` - The Voiceflow client for API interactions.
    /// * `max_connections_per_moment` - The maximum number of connections per moment.
    ///
    /// # Returns
    ///
    /// A new instance of `ClientBuilder`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// ```
    pub fn new(client_id: String, api_key: String, voiceflow_client: Arc<VoiceflowClient>, max_connections_per_moment: usize) -> Self {
        Self {
            client_id,
            api_key,
            voiceflow_client,
            sessions: None,
            max_connections_per_moment,
            connection_duration: None,
            session_duration: None,
            sessions_cleanup_interval: None,
            launch_state: State::default(),
            status: true,
            bot_auth_token: None
        }
    }

    /// Sets sessions for the client builder.
    ///
    /// # Parameters
    ///
    /// * `sessions` - A vector of sessions to set.
    ///
    /// # Returns
    ///
    /// The updated `ClientBuilder` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::session_wrappers::Session;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let sessions: Vec<Session> = vec![];
    ///
    /// let mut builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let builder = builder.set_sessions(sessions);
    /// ```
    pub fn set_sessions(mut self, sessions: Vec<Session>) -> Self {
        self.sessions = Some(sessions);
        self
    }

    /// Allows session cleaning and sets the cleanup interval.
    ///
    /// # Parameters
    ///
    /// * `interval` - The interval for session cleanup in seconds.
    ///
    /// # Returns
    ///
    /// The updated `ClientBuilder` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let mut builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let builder = builder.allow_sessions_cleaning(600);
    /// ```
    pub fn allow_sessions_cleaning(mut self, interval: u64) -> Self {
        self.sessions_cleanup_interval = Some(interval);
        self
    }

    /// Sets the session duration.
    ///
    /// # Parameters
    ///
    /// * `duration` - The duration for session validity in seconds.
    ///
    /// # Returns
    ///
    /// The updated `ClientBuilder` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let mut builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let builder = builder.set_session_duration(3600);
    /// ```
    pub fn set_session_duration(mut self, duration: i64) -> Self {
        self.session_duration = Some(duration);
        self
    }

    /// Sets the connection duration for the client builder.
    ///
    /// # Parameters
    ///
    /// * `duration` - The duration for HTTP connection validity in seconds.
    ///
    /// # Returns
    ///
    /// The updated `ClientBuilder` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let mut builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let builder = builder.set_connection_duration(120);
    /// ```
    pub fn set_connection_duration(mut self, duration: u64) -> Self {
        self.connection_duration = Some(duration);
        self
    }

    /// Sets the initial launch state for the client builder.
    ///
    /// This method allows setting the initial state of the Voiceflow interaction when the client is started.
    /// The state is used to configure the initial conditions or environment of the Voiceflow session.
    ///
    /// # Parameters
    ///
    /// * `state` - The initial state to set for the Voiceflow client interaction.
    ///
    /// # Returns
    ///
    /// The updated `ClientBuilder` instance with the new launch state set.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::{VoiceflowClient, State};
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let initial_state = State::default();
    ///
    /// let mut builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let builder = builder.set_launch_state(initial_state);
    /// ```
    pub fn set_launch_state(mut self, state: State) -> Self {
        self.launch_state = state;
        self
    }

    /// Sets the status for the client builder.
    ///
    /// # Parameters
    ///
    /// * `status` - The status indicating whether the client is active.
    ///
    /// # Returns
    ///
    /// The updated `ClientBuilder` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let mut builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let builder = builder.set_status(true);
    /// ```
    pub fn set_status(mut self, status: bool) -> Self {
        self.status = status;
        self
    }

    /// Sets the bot authentication token for the client builder.
    ///
    /// # Parameters
    ///
    /// * `bot_auth_token` - The bot authentication token.
    ///
    /// # Returns
    ///
    /// The updated `ClientBuilder` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let mut builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let builder = builder.set_bot_auth_token("new_auth_token".to_string());
    /// ```
    pub fn set_bot_auth_token(mut self, bot_auth_token: String) -> Self {
        self.bot_auth_token = Some(bot_auth_token);
        self
    }

    /// Returns the client ID.
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
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client_id = builder.client_id();
    /// ```
    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    /// Returns the API key.
    ///
    /// # Returns
    ///
    /// A reference to the API key string.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let api_key = builder.api_key();
    /// ```
    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    /// Returns the Voiceflow client.
    ///
    /// # Returns
    ///
    /// A reference to the `Arc<VoiceflowClient>`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let voiceflow_client = builder.voiceflow_client();
    /// ```
    pub fn voiceflow_client(&self) -> &Arc<VoiceflowClient> {
        &self.voiceflow_client
    }

    /// Returns the optional sessions.
    ///
    /// # Returns
    ///
    /// An `Option` containing a vector of `Session`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let sessions = builder.sessions();
    /// ```
    pub fn sessions(self) -> Option<Vec<Session>> {
        self.sessions
    }

    /// Returns the maximum number of connections per moment.
    ///
    /// # Returns
    ///
    /// A `usize` representing the maximum number of connections per moment.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let max_connections = builder.max_connections_per_moment();
    /// ```
    pub fn max_connections_per_moment(&self) -> usize {
        self.max_connections_per_moment
    }

    /// Returns the session duration.
    ///
    /// # Returns
    ///
    /// An `Option<i64>` representing the session duration in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let session_duration = builder.session_duration();
    /// ```
    pub fn session_duration(&self) -> Option<i64> {
        self.session_duration
    }

    /// Returns the connection duration.
    ///
    /// # Returns
    ///
    /// An `Option<u64>` representing the connection duration in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let connection_duration = builder.connection_duration();
    /// ```
    pub fn connection_duration(&self) -> Option<u64> {self.connection_duration}

    /// Returns the session cleanup interval.
    ///
    /// # Returns
    ///
    /// An `Option<u64>` representing the session cleanup interval in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let cleanup_interval = builder.sessions_cleanup_interval();
    /// ```
    pub fn sessions_cleanup_interval(&self) -> Option<u64> {
        self.sessions_cleanup_interval
    }

    /// Returns a reference to the current launch state of the Voiceflow client.
    ///
    /// This method allows accessing the initial state that has been set for the Voiceflow client interaction.
    /// It provides a way to retrieve the state that dictates the initial conditions or behaviors
    /// of the Voiceflow session when the client is launched.
    ///
    /// # Returns
    ///
    /// A reference to the `State` object representing the initial state of the Voiceflow client.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::{VoiceflowClient, State};
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let mut builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let initial_state = builder.launch_state();
    /// ```
    pub fn launch_state(&self) -> &State{
        &self.launch_state
    }

    /// Returns the status of the client.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the client is active.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let status = builder.status();
    /// ```
    pub fn status(&self) -> bool {
        self.status
    }

    /// Returns the bot authentication token.
    ///
    /// # Returns
    ///
    /// A reference to the optional bot authentication token.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let bot_auth_token = builder.bot_auth_token();
    /// ```
    pub fn bot_auth_token(&self) -> &Option<String> {
        &self.bot_auth_token
    }
}