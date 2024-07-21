use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use crate::core::subtypes::HttpClient;
use crate::core::voiceflow::request_structures::{ActionBuilder, ActionType, VoiceflowRequestBody, VoiceflowRequestBodyBuilder};
use crate::core::voiceflow::response_structures::VoiceflowResponse;
use crate::core::voiceflow::{State, VoiceflousionError, VoiceflowMessage, VoiceflowSession};
use crate::core::voiceflow::voiceflow_message::VoiceflowMessageBuilder;

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
    /// let vf_client = VoiceflowClient::new("api_key".to_string(), "project_id".to_string(), "version_id".to_string(), 10, Some(120));
    /// let default_duration_client = VoiceflowClient::new("api_key".to_string(), "project_id".to_string(), "version_id".to_string(), 10, None);
    /// ```
    pub fn new(voiceflow_api_key: String, project_id: String, version_id: String, max_sessions_per_moment: usize, connection_duration: Option<u64>) -> Self {
        Self {
            voiceflow_api_key,
            version_id,
            project_id,
            client: HttpClient::new(max_sessions_per_moment, connection_duration),
        }
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
    /// let max_sessions = vf_client.max_sessions_per_moment();
    /// ```
    pub fn max_sessions_per_moment(&self) -> usize {
        self.client.max_connections_per_moment()
    }

    /// Launches a dialog with the Voiceflow Bot chosen session.
    ///
    /// # Parameters
    ///
    /// * `session` - The Voiceflow session.
    /// * `state` - The optional state for variables in the bot for the session.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `VoiceflowMessage` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = vf_client.launch_dialog(&session, Some(state)).await?;
    /// ```
    pub async fn launch_dialog(&self, session: &VoiceflowSession, state: Option<State>) -> Result<VoiceflowMessage, VoiceflousionError> {
        let action = ActionBuilder::new(ActionType::Launch).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let response = self.send_stream_request(body).await;
        let voiceflow_response = response?;
        let blocks = voiceflow_response.to_blocks().await?;
        let message = VoiceflowMessageBuilder::new().build_message(blocks);
        return Ok(message?);
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
    /// A `Result` containing a `VoiceflowMessage` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = vf_client.send_message(&session, Some(state), &"Hello".to_string()).await?;
    /// ```
    pub async fn send_message(&self, session: &VoiceflowSession, state: Option<State>, text: &String) -> Result<VoiceflowMessage, VoiceflousionError> {
        let action = ActionBuilder::new(ActionType::Text).text(text.clone()).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let voiceflow_response = self.send_stream_request(body).await?;
        let blocks = voiceflow_response.to_blocks().await?;
        let message = VoiceflowMessageBuilder::new().build_message(blocks);
        return Ok(message?);
    }

    /// Sends a button selection to the Voiceflow Bot's chosen session.
    ///
    /// # Parameters
    ///
    /// * `session` - The Voiceflow session.
    /// * `state` - The optional state for variables in the bot for the session.
    /// * `text` - The text associated with the button.
    /// * `button_path` - The path of the button.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `VoiceflowMessage` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = vf_client.choose_button(&session, Some(state), &"Choice".to_string(), &"button_path".to_string()).await?;
    /// ```
    pub async fn choose_button(&self, session: &VoiceflowSession, state: Option<State>, text: &String, button_path: &String) -> Result<VoiceflowMessage, VoiceflousionError> {
        let action = ActionBuilder::new(ActionType::Path(button_path.clone())).path(text.clone()).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let voiceflow_response = self.send_stream_request(body).await?;
        let blocks = voiceflow_response.to_blocks().await?;
        let message = VoiceflowMessageBuilder::new().build_message(blocks);
        return Ok(message?);
    }

    /// Sends a request to the Voiceflow API and returns the response.
    ///
    /// # Parameters
    ///
    /// * `body` - The request body to send.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `VoiceflowResponse` or a `VoiceflousionError` if the request fails.
    async fn send_stream_request<'a>(&self, body: VoiceflowRequestBody<'a>) -> Result<VoiceflowResponse, VoiceflousionError>{
        let general_runtime_url = format!("{}/{}/{}/stream", VOICEFLOW_API_URL, &self.project_id, &self.version_id);
        let response = self.client.post(general_runtime_url)
            .header(AUTHORIZATION, &self.voiceflow_api_key)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "text/event-stream")
            .body(body.to_json()).send().await;
        match response{
            Ok(valid_response) => Ok(VoiceflowResponse::new(valid_response)),
            Err(e) => Err(VoiceflousionError::VoiceflowRequestError(self.project_id.clone(), self.version_id.clone(), e.to_string()))
        }
    }
}