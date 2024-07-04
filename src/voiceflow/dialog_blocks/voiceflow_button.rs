use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::VoiceflowError;

#[derive(Debug)]
pub(super) struct VoiceflowButton{
    action_type: VoiceflowButtonActionType,
    name: String
}
#[derive(Debug)]
pub(crate) enum VoiceflowButtonActionType{
    OpenUrl(String),
    Path
}
impl VoiceflowButton{
    pub fn new(name: String, action_type: VoiceflowButtonActionType) -> Self{
        Self{
            name,
            action_type
        }
    }
}
impl VoiceflowBlock for VoiceflowButton{}
impl FromValue for VoiceflowButton{
    type Error = VoiceflowError;

    fn from_value(value: &Value) -> Result<Self, Self::Error> {
        let name = value.get("name")
            .and_then(|name| name.as_str())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Button".to_string(), value.clone())))?
            .to_string();
        let actions = value.get("request")
            .and_then(|request| request.get("payload"))
            .and_then(|payload| payload.get("actions"))
            .and_then(|actions| actions.as_array())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Button".to_string(), value.clone())))?;

        let action_type = if let Some(action) = actions.iter().find(|action| {
                action.get("type")
                    .and_then(|action_type| action_type.as_str())
                    .map(|action_type| action_type == "open_url")
                    .unwrap_or(false)
            }) {
                let url = action
                    .get("payload")
                    .and_then(|payload| payload.get("url"))
                    .and_then(|url| url.as_str())
                    .ok_or_else(|| VoiceflowError::BlockConvertationError(("Button".to_string(), value.clone())))?
                    .to_string();
                VoiceflowButtonActionType::OpenUrl(url)
            } else {
                VoiceflowButtonActionType::Path
            };
        Ok(Self::new(name, action_type))
    }
}