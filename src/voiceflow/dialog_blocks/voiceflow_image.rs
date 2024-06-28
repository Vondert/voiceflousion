use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub(super) struct VoiceflowImage{
    url: String
}
impl FromValue for VoiceflowImage{
    type Error = VoiceflowError;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}