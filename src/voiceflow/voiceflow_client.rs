use std::time::Duration;
use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use crate::voiceflow::request_structures::{ActionBuilder, ActionType, VoiceflowRequestBody, VoiceflowRequestBodyBuilder};
use crate::voiceflow::response_structures::VoiceflowResponse;
use crate::voiceflow::{State, VoiceflousionError, VoiceflowMessage, VoiceflowSession};
use crate::voiceflow::voiceflow_message::VoiceflowMessageBuilder;

/// Voiceflow API runtime interaction url
static VOICEFLOW_API_URL: &str = "https://general-runtime.voiceflow.com/v2beta1/interact";

/// Represents a client for the Voiceflow API.
///
/// `VoiceflowClient` is used to interact with the Voiceflow API using the provided API key,
/// version ID, and project ID.
#[derive(Debug)]
pub struct VoiceflowClient{
    /// The API key for accessing Voiceflow.
    voiceflow_api_key: String,
    /// The version ID for the Voiceflow project.
    version_id: String,
    /// The project ID for the Voiceflow project.
    project_id: String,
    /// The HTTP client for making requests.
    client: Client
}
impl VoiceflowClient{
    /// Creates a new Voiceflow client.
    ///
    /// # Parameters
    ///
    /// * `voiceflow_api_key` - The API key for accessing Voiceflow.
    /// * `project_id` - The project ID for the Voiceflow project.
    /// * `version_id` - The version ID for the Voiceflow project.
    /// * `max_sessions_per_moment` - The maximum number of idle connections per host.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowClient`.
    ///
    /// # Example
    ///
    /// ```
    /// let vf_client = VoiceflowClient::new("api_key".to_string(), "project_id".to_string(), "version_id".to_string(), 10);
    /// ```
    pub fn new(voiceflow_api_key: String, project_id: String, version_id: String, max_sessions_per_moment: usize) -> Self{
        Self{
            voiceflow_api_key,
            version_id,
            project_id,
            client: Client::builder()
                .pool_max_idle_per_host(max_sessions_per_moment)
                .pool_idle_timeout(Duration::from_secs(60))
                .build().unwrap()
        }
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