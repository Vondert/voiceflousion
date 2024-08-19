use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use chrono::Utc;
use serde_json::Value;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::core::voiceflow::dialog_blocks::VoiceflowCard;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a carousel in a Voiceflow dialog.
///
/// `VoiceflowCarousel` contains a list of `VoiceflowCard` instances and a flag indicating whether the carousel has images.
/// The carousel allows for easy navigation between cards and keeps track of the selected card's index and the timestamp when it was selected.
#[derive(Debug, Clone)]
pub struct VoiceflowCarousel {
    /// The list of cards in the carousel.
    cards: Vec<VoiceflowCard>,

    /// A flag indicating whether the carousel has images.
    ///
    /// - `true`: All cards in the carousel have images.
    /// - `false`: All cards in the carousel are text-only (no images).
    /// Mixed carousels (some cards with images and others without) are not allowed and will trigger an error during initialization.
    has_images: bool,

    /// The index of the currently selected card in the carousel.
    ///
    /// This value is updated as the user navigates through the carousel.
    selected_index: Arc<AtomicUsize>,

    /// The timestamp of when the currently selected card was accessed.
    ///
    /// This value helps track when the user last interacted with a specific card.
    selected_mark: Arc<AtomicI64>,
}

impl VoiceflowCarousel {
    /// Creates a new `VoiceflowCarousel` instance.
    ///
    /// # Parameters
    ///
    /// * `cards` - A list of `VoiceflowCard` instances.
    /// * `has_images` - A flag indicating whether the carousel has images.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowCarousel` with the current timestamp and initial selected index set to 0.
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
    pub fn new(cards: Vec<VoiceflowCard>, has_images: bool) -> Self {
        let timestamp = Utc::now().timestamp();
        Self {
            cards,
            has_images,
            selected_mark: Arc::new(AtomicI64::new(timestamp)),
            selected_index: Arc::new(AtomicUsize::new(0usize)),
        }
    }

    /// Returns whether the carousel has images.
    ///
    /// # Returns
    ///
    /// `true` if the carousel has images, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let cards = vec![card];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// let has_images = carousel.has_images();
    /// ```
    pub fn has_images(&self) -> bool {
        self.has_images
    }

    /// Returns the number of cards in the carousel.
    ///
    /// # Returns
    ///
    /// A `usize` representing the number of cards in the carousel.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let cards = vec![card];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// let length = carousel.len();
    /// ```
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns whether the carousel contains any cards.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the carousel is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let cards = vec![card];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// let is_empty = carousel.is_empty();
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns the currently selected card and its index.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a tuple with a reference to the selected `VoiceflowCard` and its index.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let cards = vec![card];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// let (selected_card, index) = carousel.get_selected_card().unwrap();
    /// ```
    pub fn get_selected_card(&self) -> VoiceflousionResult<(&VoiceflowCard, usize)> {
        let index = self.get_selected_index();
        let card = self.cards.get(index).ok_or_else(|| {
            VoiceflousionError::ValidationError(
                "VoiceflousionCarousel".to_string(),
                format!("Index {} out of bounds", index),
            )
        })?;

        Ok((card, index))
    }

    /// Returns the timestamp of when the current card was selected.
    ///
    /// # Returns
    ///
    /// An `i64` representing the timestamp of the selected card.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let cards = vec![card];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// let timestamp = carousel.get_selected_mark();
    /// ```
    pub fn get_selected_mark(&self) -> i64 {
        self.selected_mark.load(Ordering::SeqCst)
    }

    /// Returns the index of the currently selected card.
    ///
    /// # Returns
    ///
    /// A `usize` representing the index of the selected card.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let cards = vec![card];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// let selected_index = carousel.get_selected_index();
    /// ```
    pub fn get_selected_index(&self) -> usize {
        self.selected_index.load(Ordering::SeqCst)
    }

    /// Retrieves the next card in the carousel based on the provided direction.
    ///
    /// # Parameters
    ///
    /// * `direction` - A boolean indicating the direction of navigation. `true` for forward, `false` for backward.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a tuple with a reference to the next `VoiceflowCard` and its index.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card1 = VoiceflowCard::new(Some("https://example.com/image1.jpg".to_string()), Some("Title1".to_string()), Some("Description1".to_string()), None);
    /// let card2 = VoiceflowCard::new(Some("https://example.com/image2.jpg".to_string()), Some("Title2".to_string()), Some("Description2".to_string()), None);
    /// let cards = vec![card1, card2];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// let (next_card, index) = carousel.get_next_card(true).unwrap(); // Navigate forward
    /// ```
    pub fn get_next_card(&self, direction: bool) -> VoiceflousionResult<(&VoiceflowCard, usize)> {
        let current_index = self.get_selected_index();
        let new_index = if direction {
            if current_index < self.cards.len() - 1 {
                current_index + 1
            } else {
                return Err(VoiceflousionError::ValidationError(
                    "VoiceflousionCarousel".to_string(),
                    format!("Index {} can't be bigger", current_index),
                ));
            }
        } else {
            if current_index > 0 {
                current_index - 1
            } else {
                return Err(VoiceflousionError::ValidationError(
                    "VoiceflousionCarousel".to_string(),
                    format!("Index {} can't be lesser", current_index),
                ));
            }
        };

        let card = self.cards.get(new_index).ok_or_else(|| {
            VoiceflousionError::ValidationError(
                "VoiceflousionCarousel".to_string(),
                format!("Index {} out of bounds", new_index),
            )
        })?;

        Ok((card, new_index))
    }

    /// Sets the selected card index and updates the timestamp of selection.
    ///
    /// # Parameters
    ///
    /// * `selected_index` - The index of the card to set as selected.
    /// * `timestamp` - The timestamp to associate with the selection.
    ///
    /// # Panics
    ///
    /// This method will panic if `selected_index` is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Utc;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowCard, VoiceflowCarousel};
    ///
    /// let card1 = VoiceflowCard::new(Some("https://example.com/image1.jpg".to_string()), Some("Title1".to_string()), Some("Description1".to_string()), None);
    /// let card2 = VoiceflowCard::new(Some("https://example.com/image2.jpg".to_string()), Some("Title2".to_string()), Some("Description2".to_string()), None);
    /// let cards = vec![card1, card2];
    /// let carousel = VoiceflowCarousel::new(cards, true);
    /// carousel.set_selected_card(1, Utc::now().timestamp());
    /// ```
    pub fn set_selected_card(&self, selected_index: usize, timestamp: i64) {
        if selected_index >= self.cards.len() {
            panic!("Index {} is out of bounds", selected_index);
        }
        self.selected_index.store(selected_index, Ordering::SeqCst);
        self.selected_mark.store(timestamp, Ordering::SeqCst);
    }
}

impl FromValue for VoiceflowCarousel {
    /// Attempts to convert a JSON `Value` into a `VoiceflowCarousel` instance.
    ///
    /// This method extracts the list of cards from the JSON value and determines
    /// whether the carousel contains only images or only text. Mixed carousels
    /// are not allowed and will return an error.
    ///
    /// # Parameters
    ///
    /// * `value` - A reference to the JSON `Value` to convert from.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing an `Option` with the `VoiceflowCarousel` instance if the conversion
    /// succeeds, or a `VoiceflousionError` if the conversion fails. If the conversion
    /// succeeds but there is no meaningful value, `None` can be returned.
    fn from_value(value: &Value) -> VoiceflousionResult<Option<Self>> {
        // Extract the "payload" field from the "trace" object in the JSON value.
        let payload = value["trace"].get("payload").ok_or_else(|| {
            VoiceflousionError::VoiceflowBlockConvertationError(
                "VoiceflowCarousel carousel payload".to_string(),
                value.clone(),
            )
        })?;

        // Extract the "cards" array from the payload.
        let cards_value = payload
            .get("cards")
            .and_then(|cards| cards.as_array())
            .ok_or_else(|| {
                VoiceflousionError::VoiceflowBlockConvertationError(
                    "VoiceflowCarousel cards value".to_string(),
                    value.clone(),
                )
            })?;

        // Convert each card in the array into a VoiceflowCard.
        let cards_option: Result<Vec<Option<VoiceflowCard>>, VoiceflousionError> = cards_value
            .into_iter()
            .map(|card| VoiceflowCard::from_value(card))
            .collect();
        let cards: Vec<VoiceflowCard> = cards_option?.into_iter().filter_map(|card| card).collect();

        // Return None if the carousel has no cards.
        if cards.is_empty() {
            return Ok(None);
        }

        // Determine if all cards have images or if all are text-only.
        let has_images = cards.iter().all(|card| card.image_url().is_some());
        let no_images = cards.iter().all(|card| card.image_url().is_none());

        // Check for mixed types: some cards with images, some without.
        if !has_images && !no_images {
            return Err(VoiceflousionError::VoiceflowBlockConvertationError(
                "VoiceflowCarousel cards value".to_string(),
                value.clone(),
            ));
        }

        // Return the constructed VoiceflowCarousel.
        Ok(Some(Self::new(cards, has_images)))
    }
}
