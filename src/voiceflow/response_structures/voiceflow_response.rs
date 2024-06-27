use std::str::Lines;
use reqwest::{Error, Response};
use serde_json::Value;
use crate::voiceflow::response_structures::voiceflow_response_type::VoiceflowResponseType;

#[derive(Debug)]
pub(crate) struct VoiceflowResponse{
    response: Response
}
impl VoiceflowResponse{
    pub(crate) fn new (response: Response) -> Self{
        Self{
            response
        }
    }
}
impl VoiceflowResponse{
    pub(crate) async fn json(self) -> Result<Vec<Value>, Error> {
        let text = self.response.text().await?;
        let events = parse_sse(text.lines());

        let mut json_values = Vec::new();
        for data in events {
            if let Ok(json) = serde_json::from_str::<Value>(&data) {
                let response_type = get_response_type(&json);
                match response_type {
                    VoiceflowResponseType::Text => {
                        json_values.push(json);
                    },
                    VoiceflowResponseType::Choice =>{
                        json_values.push(json);
                    }
                    _ => {}
                }
            }
        }

        Ok(json_values)
    }
}

fn parse_sse(lines: Lines) -> Vec<String> {
    let mut events = Vec::new();
    let mut current_data = String::new();

    for line in lines {
        if line.starts_with("data:") {
            if !current_data.is_empty() {
                current_data.push('\n');
            }
            current_data.push_str(&line[5..].trim());
        } else if line.is_empty() {
            if !current_data.is_empty() {
                events.push(current_data.clone());
                current_data.clear();
            }
        }
    }

    if !current_data.is_empty() {
        events.push(current_data);
    }
    if events.len() >= 2 {
        events = events[..(events.len() - 2)].to_vec();
    }
    events
}

fn is_trace_object (json: &Value) -> bool{
    json["trace"].is_object()
}
fn get_response_type (json: &Value) -> VoiceflowResponseType{
    if let Some(payload_type) = json.get("trace")
        .and_then(|trace| trace.get("type"))
        .and_then(|t| t.as_str())
    {
        return VoiceflowResponseType::new(payload_type)
    }
    VoiceflowResponseType::new("")
}