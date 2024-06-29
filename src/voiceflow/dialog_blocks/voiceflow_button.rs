use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::VoiceflowError;

#[derive(Debug)]
pub(super) struct VoiceflowButton{
    name: String
}
impl VoiceflowButton{
    pub fn new(name: String) -> Self{
        Self{
            name
        }
    }
}
impl VoiceflowBlock for VoiceflowButton{

}
impl FromValue for VoiceflowButton{
    type Error = VoiceflowError;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value.get("name").and_then(|name| name.as_str()){
            Some(name) => Ok(Self{
                name: name.to_string()
            }),
            None => return Err(VoiceflowError::BlockConvertationError(("Button".to_string(), value)))
        }
    }
}