use std::str::Lines;
use reqwest::Response;
use serde_json::Value;
use crate::voiceflow::response_structures::voiceflow_response_block::VoiceflowResponseBlock;
use crate::voiceflow::response_structures::voiceflow_response_block_type::VoiceflowResponseBlockType;
use crate::voiceflow::VoiceflowError;

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
    pub(crate) async fn to_blocks(self) -> Result<Vec<VoiceflowResponseBlock>, VoiceflowError> {
        let text = self.response.text().await.map_err(|error| VoiceflowError::ResponseReadingError(error.to_string()))?;
        let events = parse_sse(text.lines());

        let mut blocks = Vec::new();
        for data in events {
            if let Ok(json) = serde_json::from_str::<Value>(&data) {
                let response_type = get_response_type(&json);
                match response_type {
                    VoiceflowResponseBlockType::Text
                    | VoiceflowResponseBlockType::Choice
                    | VoiceflowResponseBlockType::CardV2
                    | VoiceflowResponseBlockType::Visual
                    | VoiceflowResponseBlockType::Carousel => {
                        let block = VoiceflowResponseBlock::new(response_type, json);
                        println!("\n Block: {:?}", &block);
                        blocks.push(block);
                    },
                    _ => {}
                }
            }
        }

        Ok(blocks)
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

fn get_response_type (json: &Value) -> VoiceflowResponseBlockType {
    if let Some(payload_type) = json.get("trace")
        .and_then(|trace| trace.get("type"))
        .and_then(|t| t.as_str())
    {
        return VoiceflowResponseBlockType::new(payload_type)
    }
    VoiceflowResponseBlockType::new("")
}