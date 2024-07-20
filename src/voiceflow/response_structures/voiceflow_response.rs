use std::str::Lines;
use reqwest::Response;
use serde_json::Value;
use crate::voiceflow::response_structures::voiceflow_response_block::VoiceflowResponseBlock;
use crate::voiceflow::response_structures::voiceflow_response_block_type::VoiceflowResponseBlockType;
use crate::voiceflow::VoiceflousionError;

/// Represents a response from the Voiceflow API.
///
/// `VoiceflowResponse` wraps the HTTP response and provides methods to process it.
#[derive(Debug)]
pub(crate) struct VoiceflowResponse {
    /// The HTTP response received from the Voiceflow API.
    response: Response,
}
impl VoiceflowResponse{
    /// Creates a new `VoiceflowResponse`.
    ///
    /// # Parameters
    ///
    /// * `response` - The HTTP response received from the Voiceflow API.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowResponse`.
    ///
    /// # Example
    ///
    /// ```
    /// let response = VoiceflowResponse::new(response);
    /// ```
    pub(crate) fn new(response: Response) -> Self {
        Self { response }
    }

    /// Converts the response text into a vector of `VoiceflowResponseBlock` instances.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `VoiceflowResponseBlock` instances or a `VoiceflousionError` if the conversion fails.
    ///
    /// # Example
    ///
    /// ```
    /// let blocks = response.to_blocks().await?;
    /// ```
    pub(crate) async fn to_blocks(self) -> Result<Vec<VoiceflowResponseBlock>, VoiceflousionError> {
        let text = self.response.text().await.map_err(|error| VoiceflousionError::VoiceflowResponseReadingError(error.to_string()))?;
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
                    | VoiceflowResponseBlockType::Carousel
                    | VoiceflowResponseBlockType::End => {
                        let block = VoiceflowResponseBlock::new(response_type, json);
                        //println!("\n Block: {:?}", &block);
                        blocks.push(block);
                    },
                    _ => {}
                }
            }
        }
        Ok(blocks)
    }
}

/// Parses Server-Sent Events (SSE) from lines of text.
///
/// # Parameters
///
/// * `lines` - An iterator over lines of text.
///
/// # Returns
///
/// A vector of strings, each representing a single SSE event.
///
/// # Example
///
/// ```
/// let events = parse_sse(text.lines());
/// ```
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

/// Determines the type of Voiceflow response block from JSON data.
///
/// # Parameters
///
/// * `json` - The JSON data representing a Voiceflow response block.
///
/// # Returns
///
/// The `VoiceflowResponseBlockType` corresponding to the JSON data.
///
/// # Example
///
/// ```
/// let block_type = get_response_type(&json);
/// ```
fn get_response_type (json: &Value) -> VoiceflowResponseBlockType {
    if let Some(payload_type) = json.get("trace")
        .and_then(|trace| trace.get("type"))
        .and_then(|t| t.as_str())
    {
        return VoiceflowResponseBlockType::new(payload_type)
    }
    VoiceflowResponseBlockType::new("")
}