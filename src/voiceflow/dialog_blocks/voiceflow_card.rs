use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::VoiceflowButtons;
use crate::voiceflow::VoiceflousionError;
#[derive(Debug)]
pub struct VoiceflowCard{
    image_url: Option<String>,
    title: String,
    description: String,
    buttons: VoiceflowButtons
}
impl VoiceflowCard{
    pub fn new(image_url: Option<String>, title: String, description: String, buttons: VoiceflowButtons) -> Self{
        Self{
            image_url,
            title,
            description,
            buttons
        }
    }
    pub fn image_url(&self) -> &Option<String>{
        &self.image_url
    }
    pub fn title(&self) -> &String{
        &self.title
    }
    pub fn description(&self) -> &String{
        &self.description
    }
    pub fn buttons(&self) -> &VoiceflowButtons{
        &self.buttons
    }
}


impl FromValue for VoiceflowCard{
    type Error = VoiceflousionError;
    fn from_value(value: &Value) -> Result<Self, Self::Error> {
        let payload = value.get("trace").and_then(|trace| trace.get("payload")).unwrap_or_else(|| value);
        let buttons: VoiceflowButtons = VoiceflowButtons::from_value(value).map_err(|_| VoiceflousionError::BlockConvertationError(("Card buttons".to_string(), value.clone())))?;

        let description = payload.get("description")
            .and_then(|description| description.get("text"))
            .and_then(|text| text.as_str())
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Card description".to_string(), value.clone())))?
            .to_string();

        let image_url_string = payload.get("imageUrl")
            .and_then(|url| url.as_str())
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Card image url".to_string(), value.clone())))?
            .to_string();
        let image_url = if image_url_string.is_empty(){
            None
        }
        else{
            Some(image_url_string)
        };
        let title = payload.get("title")
            .and_then(|title| title.as_str())
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Card title".to_string(), value.clone())))?
            .to_string();

        Ok(Self::new(image_url, title, description, buttons))
    }
}