use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::VoiceflowBlock;

pub(crate) trait FromValue: VoiceflowBlock + Sized {
    type Error;
    fn from_value(value: Value) -> Result<Self, Self::Error>;
}