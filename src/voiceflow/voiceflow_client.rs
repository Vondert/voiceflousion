use std::time::Duration;
use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use crate::voiceflow::request_structures::{ActionBuilder, ActionType, VoiceflowRequestBody, VoiceflowRequestBodyBuilder};
use crate::voiceflow::response_structures::VoiceflowResponse;
use crate::voiceflow::{State, VoiceflowError, VoiceflowMessage, VoiceflowSession};
use crate::voiceflow::voiceflow_message::VoiceflowMessageBuilder;

const VOICEFLOW_URL: &str = "https://general-runtime.voiceflow.com/v2beta1/interact";
#[derive(Debug)]
pub struct VoiceflowClient{
    voiceflow_api_key:  String,
    version_id: String,
    project_id: String,
    client: Client
}
impl VoiceflowClient{
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
    pub async fn launch_dialog(&self, session: &VoiceflowSession, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError> {
        let action = ActionBuilder::new(ActionType::Launch).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let response = self.send_stream_request(body).await;
        let voiceflow_response = response?;
        let blocks = voiceflow_response.to_blocks().await?;
        let message = VoiceflowMessageBuilder::new().build_message(blocks);
        return Ok(message?);
    }

    pub async fn send_message(&self, session: &VoiceflowSession, state: Option<State>, text: String) -> Result<VoiceflowMessage, VoiceflowError> {
        let action = ActionBuilder::new(ActionType::Text).text(text).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let voiceflow_response = self.send_stream_request(body).await?;
        let blocks = voiceflow_response.to_blocks().await?;
        let message = VoiceflowMessageBuilder::new().build_message(blocks);
        return Ok(message?);
    }

    pub async fn choose_button(&self, session: &VoiceflowSession, state: Option<State>, button_name: String) -> Result<VoiceflowMessage, VoiceflowError> {
        let action = ActionBuilder::new(ActionType::Text).text(button_name).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let voiceflow_response = self.send_stream_request(body).await?;
        let blocks = voiceflow_response.to_blocks().await?;
        let message = VoiceflowMessageBuilder::new().build_message(blocks);
        return Ok(message?);
    }
    async fn send_stream_request<'a>(&self, body: VoiceflowRequestBody<'a>) -> Result<VoiceflowResponse, VoiceflowError>{
        let general_runtime_url = format!("{}/{}/{}/stream", VOICEFLOW_URL, &self.project_id, &self.version_id);
        let response = self.client.post(general_runtime_url)
            .header(AUTHORIZATION, &self.voiceflow_api_key)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "text/event-stream")
            .body(body.to_json()).send().await;
        match response{
            Ok(valid_response) => Ok(VoiceflowResponse::new(valid_response)),
            Err(e) => Err(VoiceflowError::RequestError(e.to_string()))
        }
    }
}