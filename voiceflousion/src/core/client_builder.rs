use std::sync::Arc;
use crate::core::session_wrappers::Session;
use crate::core::voiceflow::{State, VoiceflowClient};

/// Builds a client with the necessary configurations.
///
/// `ClientBuilder` provides a builder pattern for constructing a client with various configurations,
/// including session management and cleanup settings.
pub struct ClientBuilder {
    /// The client ID.
    client_id: String,
    /// The API key for authentication.
    api_key: String,
    /// The Voiceflow client for API interactions.
    voiceflow_client: Arc<VoiceflowClient>,
    /// Optional sessions to initialize with.
    sessions: Option<Vec<Session>>,
    /// The maximum number of connections per moment.
    max_connections_per_moment: usize,
    /// Optional duration of the HTTP connection in seconds
    connection_duration: Option<u64>,
    /// The optional duration for session validity.
    session_duration: Option<i64>,
    /// The optional interval for session cleanup.
    sessions_cleanup_interval: Option<u64>,
    /// The launch state of the Voiceflow client.
    launch_state: State
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
            launch_state: State::default()
        }
    }

    /// Adds sessions to the client builder.
    ///
    /// # Parameters
    ///
    /// * `sessions` - A vector of sessions to add.
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
    /// let builder = builder.add_sessions(sessions);
    /// ```
    pub fn add_sessions(mut self, sessions: Vec<Session>) -> Self {
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
    /// let builder = builder.add_session_duration(3600);
    /// ```
    pub fn add_session_duration(mut self, duration: i64) -> Self {
        self.session_duration = Some(duration);
        self
    }


    /// Adds the connection duration to the client builder.
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
    /// let builder = builder.add_connection_duration(120);
    /// ```
    pub fn add_connection_duration(mut self, duration: u64) -> Self{
        self.connection_duration = Some(duration);
        self
    }

    /// Adds the initial launch state to the client builder.
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
    /// let builder = builder.add_launch_state(initial_state);
    /// ```
    pub fn add_launch_state(mut self, state: State) -> Self{
        self.launch_state = state;
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
}