use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use serde_json::Value;
use crate::core::subtypes::HttpClient;
use crate::core::voiceflow::request_structures::{ActionBuilder, ActionType, VoiceflowRequestBody, VoiceflowRequestBodyBuilder};
use crate::core::voiceflow::response_structures::VoiceflowResponse;
use crate::core::voiceflow::{State, VoiceflowBlock, VoiceflowMessage, VoiceflowSession};
use crate::core::voiceflow::dialog_blocks::VoiceflowText;
use crate::errors::VoiceflousionError;

/// Voiceflow API runtime interaction URL.
static VOICEFLOW_API_URL: &str = "https://general-runtime.voiceflow.com/v2beta1/interact";

/// Represents a client for the Voiceflow API.
///
/// `VoiceflowClient` is used to interact with the Voiceflow API using the provided API key,
/// version ID, and project ID.
pub struct VoiceflowClient {
    /// The API key for accessing Voiceflow.
    voiceflow_api_key: String,
    /// The version ID for the Voiceflow project.
    version_id: String,
    /// The project ID for the Voiceflow project.
    project_id: String,
    /// The HTTP client for sending requests.
    client: HttpClient,
    /// The message to return when an unexpected error occurs.
    unavailable_message: String,
    /// The message to return when the bot sends invalid response.
    invalid_response_message: String
}

impl VoiceflowClient {
    /// Creates a new Voiceflow client.
    ///
    /// # Parameters
    ///
    /// * `voiceflow_api_key` - The API key for accessing Voiceflow.
    /// * `project_id` - The project ID for the Voiceflow project.
    /// * `version_id` - The version ID for the Voiceflow project.
    /// * `max_sessions_per_moment` - The maximum number of idle connections per host.
    /// * `connection_duration` - The optional duration for which connections can remain idle (in seconds).
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let vf_client = VoiceflowClient::new("api_key".to_string(), "project_id".to_string(), "version_id".to_string(), 10, Some(120));
    /// let default_duration_client = VoiceflowClient::new("api_key".to_string(), "project_id".to_string(), "version_id".to_string(), 10, None);
    /// ```
    pub fn new(voiceflow_api_key: String, project_id: String, version_id: String, max_sessions_per_moment: usize, connection_duration: Option<u64>) -> Self {
        Self {
            voiceflow_api_key,
            version_id,
            project_id,
            client: HttpClient::new(max_sessions_per_moment, connection_duration),
            unavailable_message: "Bot is temporary unavailable".to_string(),
            invalid_response_message: "Can't read response from bot".to_string()
        }
    }

    /// Changes the message to return when an unexpected error occurs.
    ///
    /// # Parameters
    ///
    /// * `message` - The new message to return.
    ///
    /// # Returns
    ///
    /// The updated `VoiceflowClient` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// let vf_client = VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None)
    /// .change_unavailable_message("New error message".to_string());
    /// ```
    pub fn change_unavailable_message(mut self, message: String) -> Self {
        self.unavailable_message = message;
        self
    }

    /// Changes the message to return when the bot is temporarily unavailable.
    ///
    /// # Parameters
    ///
    /// * `message` - The new message to return.
    ///
    /// # Returns
    ///
    /// The updated `VoiceflowClient` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let vf_client = VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None)
    /// .change_unexpected_error_message("New unavailable message".to_string());
    /// ```
    pub fn change_unexpected_error_message(mut self, message: String) -> Self {
        self.invalid_response_message = message;
        self
    }

    /// Returns the version ID.
    ///
    /// # Returns
    ///
    /// A reference to the version ID string.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    /// let version_id = vf_client.version_id();
    /// ```
    pub fn version_id(&self) -> &String {
        &self.version_id
    }

    /// Returns the project ID.
    ///
    /// # Returns
    ///
    /// A reference to the project ID string.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    /// let project_id = vf_client.project_id();
    /// ```
    pub fn project_id(&self) -> &String {
        &self.project_id
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
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    /// let api_key = vf_client.voiceflow_api_key();
    /// ```
    pub fn voiceflow_api_key(&self) -> &String {
        &self.voiceflow_api_key
    }

    /// Returns the maximum number of idle connections per host.
    ///
    /// # Returns
    ///
    /// A `usize` representing the maximum number of idle connections per host.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    /// let max_sessions = vf_client.max_sessions_per_moment();
    /// ```
    pub fn max_sessions_per_moment(&self) -> usize {
        self.client.max_connections_per_moment()
    }

    /// Returns the message to return when an unexpected error occurs.
    ///
    /// # Returns
    ///
    /// A reference to the error message string.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    /// let error_message = vf_client.unavailable_message();
    /// ```
    pub fn unavailable_message(&self) -> &String {
        &self.unavailable_message
    }

    /// Returns the message to return when the bot is temporarily unavailable.
    ///
    /// # Returns
    ///
    /// A reference to the unavailable message string.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    /// let unavailable_message = vf_client.unexpected_error_message();
    /// ```
    pub fn unexpected_error_message(&self) -> &String {
        &self.invalid_response_message
    }

    /// Launches a dialog with the Voiceflow Bot chosen session.
    ///
    /// # Parameters
    ///
    /// * `session` - The Voiceflow session.
    /// * `state` - The state for variables in the bot for the session.
    ///
    /// # Returns
    ///
    /// A `VoiceflowMessage` containing the response from the Voiceflow API.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::session_wrappers::{LockedSession, Session};
    /// use voiceflousion::core::voiceflow::{State, VoiceflowClient, VoiceflowSession};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let session = Arc::new(Session::new("chat_id".to_string(), None, true));
    ///     let locked_session = LockedSession::try_from_session(&session)?;
    ///     let session = locked_session.voiceflow_session();
    ///
    ///     let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    ///     let state = State::default();
    ///
    ///     let response = vf_client.launch_dialog(&session, state).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn launch_dialog(&self, session: &VoiceflowSession, state: State) -> VoiceflowMessage {
        let action = ActionBuilder::new(ActionType::Launch).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(Some(state)).build();
        self.send_stream_request(body).await
    }

    /// Sends a text message to the Voiceflow Bot's chosen session.
    ///
    /// # Parameters
    ///
    /// * `session` - The Voiceflow session.
    /// * `state` - The optional state for variables in the bot for the session.
    /// * `text` - The text message to send.
    ///
    /// # Returns
    ///
    /// A `VoiceflowMessage` containing the response from the Voiceflow API.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::session_wrappers::{LockedSession, Session};
    /// use voiceflousion::core::voiceflow::{State, VoiceflowClient, VoiceflowSession};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let session = Arc::new(Session::new("chat_id".to_string(), None, true));
    ///     let locked_session = LockedSession::try_from_session(&session)?;
    ///     let session = locked_session.voiceflow_session();
    ///
    ///     let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    ///     let state = State::default();
    ///
    ///     let response = vf_client.send_message(session, Some(state), &"Hello".to_string()).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_message(&self, session: &VoiceflowSession, state: Option<State>, text: &String) -> VoiceflowMessage {
        let action = ActionBuilder::new(ActionType::Text).text(text.clone()).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        self.send_stream_request(body).await
    }

    /// Sends a button selection to the Voiceflow Bot's chosen session.
    ///
    /// # Parameters
    ///
    /// * `session` - The Voiceflow session.
    /// * `state` - The optional state for variables in the bot for the session.
    /// * `text` - The text associated with the button.
    /// * `button_path` - The path of the button.
    /// * `payload` - The payload associated with the button.
    ///
    /// # Returns
    ///
    /// A `VoiceflowMessage` containing the response from the Voiceflow API.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::session_wrappers::{LockedSession, Session};
    /// use voiceflousion::core::voiceflow::{State, VoiceflowClient, VoiceflowSession};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use serde_json::Value;
    /// let session = Arc::new(Session::new("chat_id".to_string(), None, true));
    ///     let locked_session = LockedSession::try_from_session(&session)?;
    ///     let session = locked_session.voiceflow_session();
    ///
    ///     let vf_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, None));
    ///     let state = State::default();
    ///
    ///     let response = vf_client.choose_button(&session, Some(state), &"button_path".to_string(), Value::Null).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn choose_button(&self, session: &VoiceflowSession, state: Option<State>, button_path: &String, payload: Value) -> VoiceflowMessage {
        let action = ActionBuilder::new(ActionType::Path(button_path.clone())).path(payload).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        self.send_stream_request(body).await
    }

    /// Sends a request to the Voiceflow API and returns the response.
    ///
    /// # Parameters
    ///
    /// * `body` - The request body to send.
    ///
    /// # Returns
    ///
    /// A `VoiceflowMessage` containing the response from the Voiceflow API.
    async fn send_stream_request<'a>(&self, body: VoiceflowRequestBody<'a>) -> VoiceflowMessage{
        let general_runtime_url = format!("{}/{}/{}/stream", VOICEFLOW_API_URL, &self.project_id, &self.version_id);
        let response = self.client.post(general_runtime_url)
            .header(AUTHORIZATION, &self.voiceflow_api_key)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "text/event-stream")
            .body(body.to_json()).send().await;
        let result_message = match response{
            Ok(valid_response) => {
                let voiceflow_response = VoiceflowResponse::new(valid_response);
                voiceflow_response.to_message().await
            },
            Err(e) => {
                let error = VoiceflousionError::VoiceflowRequestError(self.project_id.clone(), self.version_id.clone(), e.to_string());
                println!("{:?}", error);
                let mut message = VoiceflowMessage::default();
                message.add_block(VoiceflowBlock::Text(VoiceflowText::new(self.unavailable_message.clone())));
                Ok(message)
            }
        };
        match result_message{
            Ok(message) => message,
            Err(error) =>{
                println!("{:?}", error);
                let mut message = VoiceflowMessage::default();
                message.add_block(VoiceflowBlock::Text(VoiceflowText::new(self.invalid_response_message.clone())));
                message
            }
        }
    }
}