use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::voiceflow_image::VoiceflowImage;
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub(super) struct VoiceflowCard{
    image: VoiceflowImage,
    title: String,
    description: String
}
impl FromValue for VoiceflowCard{
    type Error = VoiceflowError;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}