use std::sync::Arc;
use crate::integrations::utils::session_wrappers::Session;
use crate::voiceflow::VoiceflowClient;

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
    /// The optional duration for session validity.
    session_duration: Option<i64>,
    /// The optional interval for session cleanup.
    sessions_cleanup_interval: Option<u64>,
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
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), Arc::new(voiceflow_client), 10);
    /// ```
    pub fn new(client_id: String, api_key: String, voiceflow_client: Arc<VoiceflowClient>, max_connections_per_moment: usize) -> Self {
        Self {
            client_id,
            api_key,
            voiceflow_client,
            sessions: None,
            max_connections_per_moment,
            session_duration: None,
            sessions_cleanup_interval: None,
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
    /// let builder = builder.add_session_duration(3600);
    /// ```
    pub fn add_session_duration(mut self, duration: i64) -> Self {
        self.session_duration = Some(duration);
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
    /// let session_duration = builder.session_duration();
    /// ```
    pub fn session_duration(&self) -> Option<i64> {
        self.session_duration
    }

    /// Returns the session cleanup interval.
    ///
    /// # Returns
    ///
    /// An `Option<u64>` representing the session cleanup interval in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let cleanup_interval = builder.sessions_cleanup_interval();
    /// ```
    pub fn sessions_cleanup_interval(&self) -> Option<u64> {
        self.sessions_cleanup_interval
    }
}