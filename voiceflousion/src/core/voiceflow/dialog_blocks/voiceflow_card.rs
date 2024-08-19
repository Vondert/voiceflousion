use serde_json::Value;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::core::voiceflow::dialog_blocks::VoiceflowButtons;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a card in a Voiceflow dialog.
///
/// `VoiceflowCard` contains optional fields for an image URL, title, description, and buttons.
#[derive(Debug, Clone)]
pub struct VoiceflowCard {
    /// The optional URL of the image.
    image_url: Option<String>,

    /// The optional title of the card.
    title: Option<String>,

    /// The optional description of the card.
    description: Option<String>,

    /// The optional buttons associated with the card.
    buttons: Option<VoiceflowButtons>,
}

impl VoiceflowCard {
    /// Creates a new `VoiceflowCard` instance.
    ///
    /// # Parameters
    ///
    /// * `image_url` - The optional URL of the image.
    /// * `title` - The optional title of the card.
    /// * `description` - The optional description of the card.
    /// * `buttons` - The optional buttons associated with the card.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowCard`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowCard;
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// ```
    pub fn new(image_url: Option<String>, title: Option<String>, description: Option<String>, buttons: Option<VoiceflowButtons>) -> Self {
        Self {
            image_url,
            title,
            description,
            buttons,
        }
    }

    /// Returns a reference to the image URL.
    ///
    /// # Returns
    ///
    /// A reference to the `Option` containing the URL string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowCard;
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let url = card.image_url();
    /// ```
    pub fn image_url(&self) -> &Option<String> {
        &self.image_url
    }

    /// Returns a reference to the title of the card.
    ///
    /// # Returns
    ///
    /// A reference to the `Option` containing the title string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowCard;
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let title = card.title();
    /// ```
    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    /// Returns a reference to the description of the card.
    ///
    /// # Returns
    ///
    /// A reference to the `Option` containing the description string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowCard;
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let description = card.description();
    /// ```
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Returns a reference to the buttons associated with the card.
    ///
    /// # Returns
    ///
    /// A reference to the `Option` containing the `VoiceflowButtons`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowCard;
    ///
    /// let card = VoiceflowCard::new(Some("https://example.com/image.jpg".to_string()), Some("Title".to_string()), Some("Description".to_string()), None);
    /// let buttons = card.buttons();
    /// ```
    pub fn buttons(&self) -> &Option<VoiceflowButtons> {
        &self.buttons
    }
}

impl FromValue for VoiceflowCard {
    /// Attempts to convert a JSON `Value` into a `VoiceflowCard` instance.
    ///
    /// This method extracts various fields such as `imageUrl`, `title`, `description`, and `buttons`
    /// from the JSON value to construct a `VoiceflowCard` instance.
    ///
    /// # Parameters
    ///
    /// * `value` - A reference to the JSON `Value` to convert from.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing an `Option` with the `VoiceflowCard` instance if the conversion
    /// succeeds, or a `VoiceflousionError` if the conversion fails. If the conversion
    /// succeeds but there is no meaningful value, `None` can be returned.
    fn from_value(value: &Value) -> VoiceflousionResult<Option<Self>> {
        // Extract the "payload" field from the "trace" object in the JSON value, or use the value itself.
        let payload = value["trace"].get("payload").unwrap_or_else(|| value);

        // Attempt to extract the buttons field and convert it into a VoiceflowButtons instance.
        let buttons: Option<VoiceflowButtons> = VoiceflowButtons::from_value(value)?;

        // Extract the description from the payload, ensuring it is a non-empty string.
        let description = payload["description"].get("text")
            .and_then(|text| text.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(
                "VoiceflowCard card description".to_string(),
                value.clone()
            ))?
            .to_string();
        let description = if description.is_empty() { None } else { Some(description) };

        // Extract the image URL from the payload, ensuring it is a non-empty string.
        let image_url = payload.get("imageUrl")
            .and_then(|url| url.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(
                "VoiceflowCard card image url".to_string(),
                value.clone()
            ))?
            .to_string();
        let image_url = if image_url.is_empty() { None } else { Some(image_url) };

        // Extract the title from the payload, ensuring it is a non-empty string.
        let title = payload.get("title")
            .and_then(|title| title.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(
                "VoiceflowCard card title".to_string(),
                value.clone()
            ))?
            .to_string();
        let mut title = if title.is_empty() { None } else { Some(title) };

        // Check if the card has meaningful content (title, buttons, description, image_url).
        match (&title, &buttons, &description, &image_url) {
            (None, None, None, None) => Ok(None),  // Return None if all fields are empty
            _ => {
                // Provide a default title if both title and description are missing
                if title.is_none() && description.is_none() {
                    title = Some(String::from("Voiceflousion placeholder card's title"));
                }
                Ok(Some(Self::new(image_url, title, description, buttons)))
            }
        }
    }
}
