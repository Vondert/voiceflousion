use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::VoiceflowButtons;
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub struct VoiceflowCard{
    image_url: String,
    title: String,
    description: String,
    buttons: VoiceflowButtons
}
impl VoiceflowCard{
    pub fn new(image_url: String, title: String, description: String, buttons: VoiceflowButtons) -> Self{
        Self{
            image_url,
            title,
            description,
            buttons
        }
    }
}


impl FromValue for VoiceflowCard{
    type Error = VoiceflowError;
    fn from_value(value: &Value) -> Result<Self, Self::Error> {
        let payload = value.get("trace").and_then(|trace| trace.get("payload")).unwrap_or_else(|| value);
        let buttons: VoiceflowButtons = VoiceflowButtons::from_value(value).map_err(|_| VoiceflowError::BlockConvertationError(("Card buttons".to_string(), value.clone())))?;

        let description = payload.get("description")
            .and_then(|description| description.get("text"))
            .and_then(|text| text.as_str())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Card description".to_string(), value.clone())))?
            .to_string();
        let image_url = payload.get("imageUrl")
            .and_then(|url| url.as_str())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Card image url".to_string(), value.clone())))?
            .to_string();
        let title = payload.get("title")
            .and_then(|title| title.as_str())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Card title".to_string(), value.clone())))?
            .to_string();

        Ok(Self::new(image_url, title, description, buttons))
    }
}