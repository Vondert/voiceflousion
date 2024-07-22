use std::ops::Deref;
use serde_json::Value;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::core::voiceflow::dialog_blocks::VoiceflowCard;
use crate::core::voiceflow::VoiceflousionError;

/// Represents a carousel in a Voiceflow dialog.
///
/// `VoiceflowCarousel` contains a list of `VoiceflowCard` instances and a flag indicating whether the carousel is full.
#[derive(Debug, Clone)]
pub struct VoiceflowCarousel {
    /// The list of cards in the carousel.
    cards: Vec<VoiceflowCard>,

    /// A flag indicating whether the carousel is full.
    is_full: bool,
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
    /// let cards = vec![VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None)];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// ```
    pub fn new(cards: Vec<VoiceflowCard>, is_full: bool) -> Self {
        Self {
            cards,
            is_full,
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
    /// let is_full = carousel.is_full();
    /// ```
    pub fn is_full(&self) -> bool {
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
    ///
    /// # Example
    ///
    /// ```
    /// let json_value = serde_json::json!({
    ///     "trace": {
    ///         "payload": {
    ///             "cards": [{
    ///                 "imageUrl": "https://example.com/image.jpg",
    ///                 "title": "Title",
    ///                 "description": {
    ///                     "text": "Description"
    ///                 },
    ///                 "buttons": [{"name": "Click me"}]
    ///             }]
    ///         }
    ///     }
    /// });
    /// let carousel = VoiceflowCarousel::from_value(&json_value)?;
    /// ```
    fn from_value(value: &Value) -> Result<Option<Self>, VoiceflousionError> {
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