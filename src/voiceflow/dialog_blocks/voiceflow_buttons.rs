use std::ops::Deref;
use serde_json::Value;
use crate::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::VoiceflowButton;
use crate::voiceflow::VoiceflousionError;
#[derive(Debug, Clone)]
pub struct VoiceflowButtons{
    option: VoiceflowButtonsOption,
    buttons: Vec<VoiceflowButton>
}
impl VoiceflowButtons{
    pub fn new(buttons: Vec<VoiceflowButton>) -> Self{
        Self{
            buttons,
            option: VoiceflowButtonsOption::Empty
        }
    }
    pub fn option(&self) -> &VoiceflowButtonsOption{
        &self.option
    }
    pub fn set_option(&mut self,  buttons_option: VoiceflowButtonsOption) -> (){
        self.option = buttons_option;
    }
}
impl Deref for VoiceflowButtons{
    type Target = Vec<VoiceflowButton>;

    fn deref(&self) -> &Self::Target {
        &self.buttons
    }
}

impl FromValue for VoiceflowButtons{
    type Error = VoiceflousionError;
    fn from_value(value: &Value) -> Result<Option<Self>, Self::Error> {

        let buttons_value = match value.get("trace").and_then(|trace| trace.get("payload"))
            .and_then(|payload| payload.get("buttons")){
            None => value.get("buttons").and_then(|buttons| buttons.as_array()),
            Some(buttons) => buttons.as_array()
        }.ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButtons buttons value".to_string(), value.clone())))?;


        let buttons_option: Result<Vec<Option<VoiceflowButton>>, Self::Error> = buttons_value.into_iter()
            .map(|button| VoiceflowButton::from_value(button))
            .collect();
        let buttons: Vec<VoiceflowButton> = buttons_option?.into_iter().filter_map(|button| button).collect();
        if buttons.is_empty(){
            return Ok(None)
        }
       Ok(Some(Self::new(buttons)))
    }
}