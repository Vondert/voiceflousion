use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU8, Ordering};
use chrono::Utc;
use serde_json::Value;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::core::voiceflow::dialog_blocks::VoiceflowCard;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a carousel in a Voiceflow dialog.
///
/// `VoiceflowCarousel` contains a list of `VoiceflowCard` instances and a flag indicating whether the carousel is full.
#[derive(Debug, Clone)]
pub struct VoiceflowCarousel {
    /// The list of cards in the carousel.
    cards: Vec<VoiceflowCard>,

    /// A flag indicating whether the carousel is full.
    is_full: bool,

    selected_index: Arc<AtomicU8>,
    selected_mark: Arc<AtomicI64>
}

impl VoiceflowCarousel {
    /// Creates a new `VoiceflowCarousel` instance.
    ///
    /// # Parameters
    ///
    /// * `cards` - A list of `VoiceflowCard` instances.
    /// * `is_full` - A flag indicating whether the carousel is full.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowCarousel`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let cards = vec![card];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// ```
    pub fn new(cards: Vec<VoiceflowCard>, is_full: bool) -> Self {
        let timestamp = Utc::now().timestamp();
        Self {
            cards,
            is_full,
            selected_mark: Arc::new(AtomicI64::new(timestamp)),
            selected_index: Arc::new(AtomicU8::new(0u8))
        }
    }

    /// Returns whether the carousel is full.
    ///
    /// # Returns
    ///
    /// `true` if the carousel is full, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let cards = vec![card];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// let is_full = carousel.is_full();
    /// ```
    pub fn is_full(&self) -> bool {
        self.is_full
    }

    pub fn get_selected_card(&self) -> VoiceflousionResult<(&VoiceflowCard, usize)>{
        let index = self.get_selected_index();
        let card = self.cards.get(index)
            .ok_or_else(|| VoiceflousionError::ValidationError("VoiceflousionCarousel".to_string(), format!("Index {} out of bounds", index)))?;

        Ok((card, index))
    }
    pub fn get_selected_mark(&self) -> i64{
        self.selected_mark.load(Ordering::SeqCst)
    }
    pub fn get_selected_index(&self) -> usize{
        self.selected_index.load(Ordering::SeqCst) as usize
    }

    pub fn shift_and_get_card(&self, direction: bool) -> VoiceflousionResult<(&VoiceflowCard, usize)> {
        let current_index = self.get_selected_index();
        let new_index = if direction {
            if current_index < self.cards.len() - 1 {
                current_index + 1
            } else {
                return Err(VoiceflousionError::ValidationError("VoiceflousionCarousel".to_string(), format!("Index {} can't be bigger", current_index)))
            }
        } else {
            if current_index > 0 {
                current_index - 1
            } else {
                return Err(VoiceflousionError::ValidationError("VoiceflousionCarousel".to_string(), format!("Index {} can't be lesser", current_index)))
            }
        };


        let card = self.cards.get(new_index)
            .ok_or_else(|| VoiceflousionError::ValidationError("VoiceflousionCarousel".to_string(), format!("Index {} out of bounds", new_index)))?;

        self.selected_index.store(new_index as u8, Ordering::SeqCst);
        self.selected_mark.store(Utc::now().timestamp(), Ordering::SeqCst);

        Ok((card, new_index))
    }
}
impl Deref for VoiceflowCarousel{
    type Target = Vec<VoiceflowCard>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl FromValue for VoiceflowCarousel{

    /// Attempts to convert a JSON `Value` into a `VoiceflowCarousel` instance.
    ///
    /// # Parameters
    ///
    /// * `value` - A reference to the JSON `Value` to convert from.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the `VoiceflowCarousel` instance if the conversion
    /// succeeds, or a `VoiceflousionError` if the conversion fails. If the conversion
    /// succeeds but there is no meaningful value, `None` can be returned.
    fn from_value(value: &Value) -> VoiceflousionResult<Option<Self>> {
        let payload = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowCarousel carousel payload".to_string(), value.clone())))?;

        let cards_value = payload.get("cards").and_then(|cards| cards.as_array())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowCarousel cards value".to_string(), value.clone())))?;

        let cards_option: Result<Vec<Option<VoiceflowCard>>, VoiceflousionError> = cards_value.into_iter().map(|card| VoiceflowCard::from_value(card)).collect();
        let cards: Vec<VoiceflowCard> = cards_option?.into_iter().filter_map(|card| card).collect();

        if cards.is_empty(){
            return Ok(None)
        }
        let is_full = cards.iter().all(|card| card.image_url().is_some());
        Ok(Some(Self::new(cards, is_full)))
    }
}