use std::ops::Deref;
use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::dialog_blocks::voiceflow_button::VoiceflowButton;
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub(super) struct VoiceflowButtons{
    buttons: Vec<VoiceflowButton>
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
    fn from_value(value: Value) -> Result<Self, Self::Error> {
        let buttons_value = if let Some(json_buttons) = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .and_then(|payload| payload.get("buttons"))
            .and_then(|buttons| buttons.as_array())
        {
            json_buttons.to_owned()
        }
        else{
            return Err(VoiceflowError::BlockConvertationError(("Buttons".to_string(), value)))
        };

        let buttons: Result<Vec<VoiceflowButton>, Self::Error> = buttons_value.into_iter()
            .map(VoiceflowButton::from_value)
            .collect();
        let buttons = buttons?;

       Ok(Self{
           buttons
       })
    }
}