use serde_json::Value;
use crate::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::VoiceflousionError;

#[derive(Debug)]
pub struct VoiceflowButton{
    action_type: VoiceflowButtonActionType,
    name: String
}
impl VoiceflowButton{
    pub fn new(name: String, action_type: VoiceflowButtonActionType) -> Self{
        Self{
            name,
            action_type
        }
    }
    pub fn action_type(&self) -> &VoiceflowButtonActionType{
        &self.action_type
    }
    pub fn name(&self) -> &String{
        &self.name
    }
}
impl FromValue for VoiceflowButton{
    type Error = VoiceflousionError;

    fn from_value(value: &Value) -> Result<Self, Self::Error> {
        let name = value.get("name")
            .and_then(|name| name.as_str())
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Button".to_string(), value.clone())))?
            .to_string();
        let actions = value.get("request")
            .and_then(|request| request.get("payload"))
            .and_then(|payload| payload.get("actions"))
            .and_then(|actions| actions.as_array())
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Button".to_string(), value.clone())))?;

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
                    .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Button".to_string(), value.clone())))?
                    .to_string();
                VoiceflowButtonActionType::OpenUrl(url)
            } else {
                VoiceflowButtonActionType::Path
            };
        Ok(Self::new(name, action_type))
    }
}