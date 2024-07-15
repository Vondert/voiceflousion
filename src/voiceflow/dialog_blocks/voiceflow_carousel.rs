use std::ops::Deref;
use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::VoiceflowCard;
use crate::voiceflow::VoiceflousionError;
#[derive(Debug, Clone)]
pub struct VoiceflowCarousel{
    cards: Vec<VoiceflowCard>,
    is_full: bool
}

impl VoiceflowCarousel{
    pub fn new(cards: Vec<VoiceflowCard>, is_full: bool) -> Self{
        Self{
            cards,
            is_full
        }
    }
    pub fn is_full(&self) -> bool{
        self.is_full
    }
}
impl Deref for VoiceflowCarousel{
    type Target = Vec<VoiceflowCard>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl FromValue for VoiceflowCarousel{
    type Error = VoiceflousionError;
    fn from_value(value: &Value) -> Result<Option<Self>, Self::Error> {
        let payload = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Carousel payload".to_string(), value.clone())))?;

        let cards_value = payload.get("cards").and_then(|cards| cards.as_array())
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Carousel Cards".to_string(), value.clone())))?;

        let cards_option: Result<Vec<Option<VoiceflowCard>>, Self::Error> = cards_value.into_iter().map(|card| VoiceflowCard::from_value(card)).collect();
        let cards: Vec<VoiceflowCard> = cards_option?.into_iter().filter_map(|card| card).collect();
        if cards.is_empty(){
            return Ok(None)
        }
        let is_full = cards.iter().all(|card| card.image_url().is_some());
        Ok(Some(Self::new(cards, is_full)))
    }
}