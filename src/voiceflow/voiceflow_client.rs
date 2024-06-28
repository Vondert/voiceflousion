use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use crate::voiceflow::dialog_blocks::{VoiceflowMessage, VoiceflowMessageBuilder};
use crate::voiceflow::request_structures::{ActionBuilder, ActionType, Session, State, VoiceflowRequestBody, VoiceflowRequestBodyBuilder};
use crate::voiceflow::response_structures::VoiceflowResponse;
use crate::voiceflow::VoiceflowError;

#[derive(Debug)]
pub struct VoiceflowClient{
    voiceflow_api_key:  String,
    version_id: String,
    project_id: String,
    general_runtime_url: String
}
impl VoiceflowClient{
    pub fn new(voiceflow_api_key: String, project_id: String, version_id: String) -> Self{
        Self{
            general_runtime_url: format!("https://general-runtime.voiceflow.com/v2beta1/interact/{}/{}/stream", &project_id, &version_id),
            voiceflow_api_key,
            version_id,
            project_id
        }
    }
    pub async fn launch_dialog(&self, session: &Session, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError> {
        let action = ActionBuilder::new(ActionType::Launch).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let response = self.send_stream_request(body).await;
        let voiceflow_response = response?;
        let blocks = voiceflow_response.to_blocks().await?;
        //println!("Response: {:?}", blocks);
        let message = VoiceflowMessageBuilder::new().build_message(blocks)?;
        println!("{:?}", message);
        Ok(message)
    }

    pub async fn send_message(&self, session: &Session, state: Option<State>, text: String) -> Result<(), VoiceflowError> {
        let action = ActionBuilder::new(ActionType::Text).text(text).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let voiceflow_response = self.send_stream_request(body).await?;
        let blocks = voiceflow_response.to_blocks().await?;
        //println!("Response: {:?}", json);
        Ok(())
    }

    pub async fn choose_button(&self, session: &Session, state: Option<State>, button_name: String) -> Result<(), VoiceflowError> {
        let action = ActionBuilder::new(ActionType::Text).text(button_name).build();
        let body = VoiceflowRequestBodyBuilder::new(action).session(Some(session)).state(state).build();
        let voiceflow_response = self.send_stream_request(body).await?;
        let blocks = voiceflow_response.to_blocks().await?;
        //println!("Response: {:?}", json);
        Ok(())
    }
    async fn send_stream_request<'a>(&self, body: VoiceflowRequestBody<'a>) -> Result<VoiceflowResponse, VoiceflowError>{
        let client = Client::new();
        let response = client.post(&self.general_runtime_url)
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