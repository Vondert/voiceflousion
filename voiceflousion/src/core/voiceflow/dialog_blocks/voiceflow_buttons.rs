use std::ops::Deref;
use chrono::Utc;
use serde_json::Value;
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::core::voiceflow::dialog_blocks::{VoiceflowButton, VoiceflowText};
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a collection of buttons in a Voiceflow dialog.
///
/// `VoiceflowButtons` contains a list of `VoiceflowButton` instances, an optional
/// buttons option, and a timestamp indicating when the buttons were marked.
#[derive(Debug, Clone)]
pub struct VoiceflowButtons {
    /// The optional buttons option providing additional context or data.
    option: VoiceflowButtonsOption,

    /// The list of buttons.
    buttons: Vec<VoiceflowButton>,

    /// The timestamp marking when the buttons were created.
    mark_timestamp: i64,
}

impl VoiceflowButtons {
    /// Creates a new `VoiceflowButtons` instance.
    ///
    /// # Parameters
    ///
    /// * `buttons` - A list of `VoiceflowButton` instances.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowButtons`.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowButton, VoiceflowButtons};
    ///
    /// let buttons = vec![VoiceflowButton::new("Click me".to_string(), VoiceflowButtonActionType::Path, Value::Null)];
    /// let voiceflow_buttons = VoiceflowButtons::new(buttons);
    /// ```
    pub fn new(buttons: Vec<VoiceflowButton>) -> Self {
        Self {
            buttons,
            mark_timestamp: Utc::now().timestamp(),
            option: VoiceflowButtonsOption::Text(VoiceflowText::new(String::from("Voiceflousion placeholder button's text"))),
        }
    }

    /// Returns a reference to the buttons option.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowButtonsOption`.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowButton, VoiceflowButtons};
    ///
    /// let buttons = vec![VoiceflowButton::new("Click me".to_string(), VoiceflowButtonActionType::Path, Value::Null)];
    /// let voiceflow_buttons = VoiceflowButtons::new(buttons);
    ///
    /// let option = voiceflow_buttons.option();
    /// ```
    pub fn option(&self) -> &VoiceflowButtonsOption {
        &self.option
    }

    /// Returns the timestamp marking when the buttons were created.
    ///
    /// # Returns
    ///
    /// An `i64` representing the creation timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowButtons;
    ///
    /// let buttons = VoiceflowButtons::new(vec![]);
    /// let mark = buttons.mark();
    /// ```
    pub fn mark(&self) -> i64 {
        self.mark_timestamp
    }

    /// Sets the buttons option.
    ///
    /// # Parameters
    ///
    /// * `buttons_option` - The new `VoiceflowButtonsOption`.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::{VoiceflowButtonActionType, VoiceflowButtonsOption};
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowButton, VoiceflowButtons, VoiceflowText};
    ///
    /// let buttons = vec![VoiceflowButton::new("Click me".to_string(), VoiceflowButtonActionType::Path, Value::Null)];
    /// let mut voiceflow_buttons = VoiceflowButtons::new(buttons);
    ///
    /// voiceflow_buttons.set_option(VoiceflowButtonsOption::Text(VoiceflowText::new("Option text".to_string())));
    /// ```
    pub fn set_option(&mut self, buttons_option: VoiceflowButtonsOption) {
        self.option = buttons_option;
    }

    /// Returns a reference to the button at the specified index.
    ///
    /// # Parameters
    ///
    /// * `button_index` - The index of the button to retrieve.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a reference to the `VoiceflowButton`, or an error if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowButton, VoiceflowButtons};
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    ///
    /// let buttons = vec![VoiceflowButton::new("Click me".to_string(), VoiceflowButtonActionType::Path, Value::Null)];
    /// let voiceflow_buttons = VoiceflowButtons::new(buttons);
    ///
    /// let button = voiceflow_buttons.get_button(0).unwrap();
    /// ```
    pub fn get_button(&self, button_index: usize) -> VoiceflousionResult<&VoiceflowButton> {
        self.get(button_index).ok_or_else(
            || VoiceflousionError::ValidationError("SentMessage content".to_string(), format!("Invalid index {} for buttons container with {} buttons", button_index, self.len()))
        )
    }
}

impl Deref for VoiceflowButtons{
    type Target = Vec<VoiceflowButton>;

    fn deref(&self) -> &Self::Target {
        &self.buttons
    }
}

impl FromValue for VoiceflowButtons{

    /// Attempts to convert a JSON `Value` into a `VoiceflowButtons` instance.
    ///
    /// # Parameters
    ///
    /// * `value` - A reference to the JSON `Value` to convert from.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the `VoiceflowButtons` instance if the conversion
    /// succeeds, or a `VoiceflousionError` if the conversion fails. If the conversion
    /// succeeds but there are no buttons, `None` can be returned.
    fn from_value(value: &Value) -> VoiceflousionResult<Option<Self>> {

        let buttons_value = match value.get("trace").and_then(|trace| trace.get("payload"))
            .and_then(|payload| payload.get("buttons")){
            None => value.get("buttons").and_then(|buttons| buttons.as_array()),
            Some(buttons) => buttons.as_array()
        }.ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButtons buttons value".to_string(), value.clone())))?;


        let buttons_option: Result<Vec<Option<VoiceflowButton>>, VoiceflousionError> = buttons_value.into_iter()
            .map(|button| VoiceflowButton::from_value(button))
            .collect();
        let buttons: Vec<VoiceflowButton> = buttons_option?.into_iter().filter_map(|button| button).collect();
        if buttons.is_empty(){
            return Ok(None)
        }

       Ok(Some(Self::new(buttons)))
    }
}