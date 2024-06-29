use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub(super) struct VoiceflowText{
    message: String
}

impl VoiceflowBlock for VoiceflowText {}

impl FromValue for VoiceflowText{
    type Error = VoiceflowError;
    fn from_value(value: Value) -> Result<Self, Self::Error> {
        if let Some(message) = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .and_then(|payload| payload.get("message"))
            .and_then(|message| message.as_str())
        {
            return Ok(Self {
                message: message.to_string()
            });
        }
        Err(VoiceflowError::BlockConvertationError(("Text".to_string(), value)))
    }
}