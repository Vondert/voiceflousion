use reqwest::{Client, Error, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use crate::voiceflow::request_structures::{ActionType, Session, State, VoiceflowRequestBody, VoiceflowRequestBodyBuilder};
use crate::voiceflow::response_structures::VoiceflowResponse;

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
    pub async fn launch_dialog(&self, session: Option<Session>, state: Option<State>) -> Result<(), Error> {
        let body = VoiceflowRequestBodyBuilder::new(ActionType::Launch).session(session).state(state).build();
        let response = self.send_stream_request(body).await;
        let voiceflow_response = response?;
        let json = voiceflow_response.json().await;
        println!("Response: {:?}", json);
        Ok(())
    }
    async fn send_stream_request (&self, body: VoiceflowRequestBody) -> Result<VoiceflowResponse, Error>{
        let client = Client::new();
        let response = VoiceflowResponse::new(client.post(&self.general_runtime_url)
            .header(AUTHORIZATION, &self.voiceflow_api_key)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "text/event-stream")
            .body(body.to_json()).send().await?);

        Ok(response)
    }
}