use std::ops::Deref;
use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::voiceflow_card::VoiceflowCard;
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub(super) struct VoiceflowCarousel{
    cards: Vec<VoiceflowCard>
}

impl Deref for VoiceflowCarousel{
    type Target = Vec<VoiceflowCard>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}
impl FromValue for VoiceflowCarousel{
    type Error = VoiceflowError;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}