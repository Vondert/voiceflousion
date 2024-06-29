use std::ops::Deref;
use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::dialog_blocks::voiceflow_card::VoiceflowCard;
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub(super) struct VoiceflowCarousel{
    cards: Vec<VoiceflowCard>
}

impl VoiceflowCarousel{
    pub fn new(cards: Vec<VoiceflowCard>) -> Self{
        Self{
            cards
        }
    }
}
impl Deref for VoiceflowCarousel{
    type Target = Vec<VoiceflowCard>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl VoiceflowBlock for VoiceflowCarousel {}

impl FromValue for VoiceflowCarousel{
    type Error = VoiceflowError;
    fn from_value(value: &Value) -> Result<Self, Self::Error> {
        let payload = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Carousel payload".to_string(), value.clone())))?;

        let cards_value = payload.get("cards").and_then(|cards| cards.as_array())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Carousel Cards".to_string(), value.clone())))?;

        let cards: Result<Vec<VoiceflowCard>, Self::Error> = cards_value.into_iter().map(|card| VoiceflowCard::from_value(card)).collect();
        let cards = cards?;

        Ok(Self::new(cards))
    }
}