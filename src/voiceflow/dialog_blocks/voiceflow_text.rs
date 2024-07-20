use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::VoiceflousionError;
/// Represents a text message block in a Voiceflow response.
///
/// `VoiceflowText` contains the message string extracted from a Voiceflow response.
#[derive(Debug, Clone)]
pub struct VoiceflowText {
    /// The message string of the text block.
    message: String,
}
impl VoiceflowText{
    pub fn new(message: String) -> Self{
        Self{
            message
        }
    }
    pub fn message(&self) -> &String{
        &self.message
    }
}

impl FromValue for VoiceflowText{
    fn from_value(value: &Value) -> Result<Option<Self>, VoiceflousionError> {
        let message = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .and_then(|payload| payload.get("message"))
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowText text message".to_string(), value.clone())))?
            .as_str()
            .map(|s| s.trim_matches('"').to_string())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowText text message".to_string(), value.clone())))?;

        if message.is_empty(){
            return Ok(None)
        }
        Ok(Some(Self::new(message)))
    }
}