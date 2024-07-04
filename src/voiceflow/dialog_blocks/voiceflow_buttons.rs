use std::ops::Deref;
use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::dialog_blocks::voiceflow_button::VoiceflowButton;
use crate::voiceflow::dialog_blocks::voiceflow_image::VoiceflowImage;
use crate::voiceflow::dialog_blocks::voiceflow_text::VoiceflowText;
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub(super) struct VoiceflowButtons{
    option: VoiceflowButtonsOption,
    buttons: Vec<VoiceflowButton>
}
#[derive(Debug)]
pub(super) enum VoiceflowButtonsOption{
    Text(VoiceflowText),
    Image(VoiceflowImage),
    Empty
}
impl VoiceflowButtons{
    pub fn new(buttons: Vec<VoiceflowButton>) -> Self{
        Self{
            buttons,
            option: VoiceflowButtonsOption::Empty
        }
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

impl VoiceflowBlock for VoiceflowButtons {}

impl FromValue for VoiceflowButtons{
    type Error = VoiceflowError;
    fn from_value(value: &Value) -> Result<Self, Self::Error> {

        let buttons_value = match value.get("trace").and_then(|trace| trace.get("payload"))
            .and_then(|payload| payload.get("buttons")){
            None => value.get("buttons").and_then(|buttons| buttons.as_array()),
            Some(buttons) => buttons.as_array()
        }.ok_or_else(|| VoiceflowError::BlockConvertationError(("Buttons".to_string(), value.clone())))?;


        let buttons: Result<Vec<VoiceflowButton>, Self::Error> = buttons_value.into_iter()
            .map(|button| VoiceflowButton::from_value(button))
            .collect();
        let buttons = buttons?;

       Ok(Self::new(buttons))
    }
}