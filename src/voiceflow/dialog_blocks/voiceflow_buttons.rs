use std::ops::Deref;
use serde_json::Value;
use crate::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::VoiceflowButton;
use crate::voiceflow::VoiceflousionError;

/// Represents a collection of buttons in a Voiceflow dialog.
///
/// `VoiceflowButtons` contains a list of `VoiceflowButton` instances and an optional
/// buttons option to provide additional data.
#[derive(Debug, Clone)]
pub struct VoiceflowButtons {
    /// The optional buttons option providing additional context or data.
    option: VoiceflowButtonsOption,

    /// The list of buttons.
    buttons: Vec<VoiceflowButton>,
}
impl VoiceflowButtons{
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
    /// let buttons = vec![VoiceflowButton::new("Click me".to_string(), "/path".to_string(), VoiceflowButtonActionType::Path)];
    /// let voiceflow_buttons = VoiceflowButtons::new(buttons);
    /// ```
    pub fn new(buttons: Vec<VoiceflowButton>) -> Self {
        Self {
            buttons,
            option: VoiceflowButtonsOption::Empty,
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
    /// let option = voiceflow_buttons.option();
    /// ```
    pub fn option(&self) -> &VoiceflowButtonsOption {
        &self.option
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
    /// voiceflow_buttons.set_option(VoiceflowButtonsOption::Text(VoiceflowText::new("Option text".to_string())));
    /// ```
    pub fn set_option(&mut self, buttons_option: VoiceflowButtonsOption) {
        self.option = buttons_option;
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
    ///
    /// # Example
    ///
    /// ```
    /// let json_value = serde_json::json!({
    ///     "trace": {
    ///         "payload": {
    ///             "buttons": [{"name": "Click me"}]
    ///         }
    ///     }
    /// });
    /// let buttons = VoiceflowButtons::from_value(&json_value)?;
    /// ```
    fn from_value(value: &Value) -> Result<Option<Self>, VoiceflousionError> {

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