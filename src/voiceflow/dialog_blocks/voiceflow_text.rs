use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::VoiceflousionError;
#[derive(Debug)]
pub struct VoiceflowText{
    message: String
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
    type Error = VoiceflousionError;
    fn from_value(value: &Value) -> Result<Self, Self::Error> {
        let message = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .and_then(|payload| payload.get("message"))
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Text".to_string(), value.clone())))?
            .as_str()
            .map(|s| s.trim_matches('"').to_string())
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Text".to_string(), value.clone())))?;

        Ok(Self::new(message))
    }
}