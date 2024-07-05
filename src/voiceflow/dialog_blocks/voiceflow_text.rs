use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::VoiceflowError;
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
}

impl FromValue for VoiceflowText{
    type Error = VoiceflowError;
    fn from_value(value: &Value) -> Result<Self, Self::Error> {
        let message = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .and_then(|payload| payload.get("message"))
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Text".to_string(), value.clone())))?
            .to_string();

        Ok(Self::new(message.to_string()))
    }
}