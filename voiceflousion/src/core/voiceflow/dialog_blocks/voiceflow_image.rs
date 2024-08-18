use serde_json::Value;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents an image in a Voiceflow dialog.
///
/// `VoiceflowImage` contains the URL and optional dimensions (height and width) of the image.
#[derive(Debug, Clone)]
pub struct VoiceflowImage {
    /// The URL of the image.
    url: String,

    /// The optional height of the image.
    height: Option<u64>,

    /// The optional width of the image.
    width: Option<u64>,
}

impl VoiceflowImage {
    /// Creates a new `VoiceflowImage` instance.
    ///
    /// # Parameters
    ///
    /// * `url` - The URL of the image.
    /// * `height` - The optional height of the image.
    /// * `width` - The optional width of the image.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowImage`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowImage;
    ///
    /// let image = VoiceflowImage::new("https://example.com/image.jpg".to_string(), Some(100), Some(200));
    /// ```
    pub fn new(url: String, height: Option<u64>, width: Option<u64>) -> Self {
        Self {
            url,
            height,
            width,
        }
    }

    /// Returns a reference to the URL of the image.
    ///
    /// # Returns
    ///
    /// A reference to the URL string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowImage;
    ///
    /// let image = VoiceflowImage::new("https://example.com/image.jpg".to_string(), Some(100), Some(200));
    /// let url = image.url();
    /// ```
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Returns the height of the image, if available.
    ///
    /// # Returns
    ///
    /// An `Option` containing the height of the image, if available.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowImage;
    ///
    /// let image = VoiceflowImage::new("https://example.com/image.jpg".to_string(), Some(100), Some(200));
    /// let height = image.height();
    /// ```
    pub fn height(&self) -> Option<u64> {
        self.height
    }

    /// Returns the width of the image, if available.
    ///
    /// # Returns
    ///
    /// An `Option` containing the width of the image, if available.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowImage;
    ///
    /// let image = VoiceflowImage::new("https://example.com/image.jpg".to_string(), Some(100), Some(200));
    /// let width = image.width();
    /// ```
    pub fn width(&self) -> Option<u64> {
        self.width
    }
}


impl FromValue for VoiceflowImage{
    /// Attempts to convert a JSON `Value` into a `VoiceflowImage` instance.
    ///
    /// # Parameters
    ///
    /// * `value` - A reference to the JSON `Value` to convert from.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the `VoiceflowImage` instance if the conversion
    /// succeeds, or a `VoiceflousionError` if the conversion fails. If the conversion
    /// succeeds but there is no meaningful value, `None` can be returned.
    fn from_value(value: &Value) -> VoiceflousionResult<Option<Self>> {
        let payload = value["trace"].get("payload")
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowImage image payload".to_string(), value.clone())))?;

        let url = payload.get("image")
            .and_then(|image| image.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowImage image url".to_string(), value.clone())))?
            .to_string();

        let height = payload["dimensions"].get("height")
            .and_then(|height| height.as_u64());

        let width = payload["dimensions"].get("width")
            .and_then(|width| width.as_u64());
        if url.is_empty() {
            return Ok(None)
        }
        Ok(Some(Self::new(url, height, width)))
    }
}