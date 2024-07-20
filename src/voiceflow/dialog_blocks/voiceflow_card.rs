use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::VoiceflowButtons;
use crate::voiceflow::VoiceflousionError;
#[derive(Debug, Clone)]
pub struct VoiceflowCard{
    image_url: Option<String>,
    title: Option<String>,
    description: Option<String>,
    buttons: Option<VoiceflowButtons>
}
impl VoiceflowCard{
    pub fn new(image_url: Option<String>, title: Option<String>, description: Option<String>, buttons: Option<VoiceflowButtons>) -> Self{
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
    pub fn title(&self) -> &Option<String>{
        &self.title
    }
    pub fn description(&self) -> &Option<String>{
        &self.description
    }
    pub fn buttons(&self) -> &Option<VoiceflowButtons> {
        &self.buttons
    }
}


impl FromValue for VoiceflowCard{
    type Error = VoiceflousionError;
    fn from_value(value: &Value) -> Result<Option<Self>, Self::Error> {
        let payload = value.get("trace").and_then(|trace| trace.get("payload")).unwrap_or_else(|| value);
        let buttons: Option<VoiceflowButtons> = VoiceflowButtons::from_value(value)?;

        let description = payload.get("description")
            .and_then(|description| description.get("text"))
            .and_then(|text| text.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowCard card description".to_string(), value.clone())))?
            .to_string();
        let description = if description.is_empty(){
            None
        }
        else{
            Some(description)
        };

        let image_url = payload.get("imageUrl")
            .and_then(|url| url.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowCard card image url".to_string(), value.clone())))?
            .to_string();
        let image_url = if image_url.is_empty(){
            None
        }
        else{
            Some(image_url)
        };

        let title = payload.get("title")
            .and_then(|title| title.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowCard card title".to_string(), value.clone())))?
            .to_string();
        let title = if  title.is_empty(){
            None
        }
        else{
          Some(title)
        };
        match (&image_url, &title, &description){
            (None, None, None) => {
                Ok(None)
            },
            _ =>  Ok(Some(Self::new(image_url, title, description, buttons)))
        }
    }
}