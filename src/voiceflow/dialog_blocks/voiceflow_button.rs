use serde_json::Value;
use crate::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::VoiceflousionError;

/// Represents a button in a Voiceflow dialog.
///
/// `VoiceflowButton` contains the name, path, and action type of button.
#[derive(Debug, Clone)]
pub struct VoiceflowButton {
    /// The action type of the button.
    action_type: VoiceflowButtonActionType,

    /// The path associated with the button.
    path: String,

    /// The name of the button.
    name: String,
}
impl VoiceflowButton{
    /// Creates a new `VoiceflowButton` instance.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the button.
    /// * `path` - The path associated with the button.
    /// * `action_type` - The action type of the button.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowButton`.
    ///
    /// # Example
    ///
    /// ```
    /// let button = VoiceflowButton::new("Button 1".to_string(), "/path".to_string(), VoiceflowButtonActionType::Path);
    /// ```
    pub fn new(name: String, path: String, action_type: VoiceflowButtonActionType) -> Self {
        Self {
            name,
            path,
            action_type,
        }
    }

    /// Returns a reference to the action type of the button.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowButtonActionType`.
    ///
    /// # Example
    ///
    /// ```
    /// let action_type = button.action_type();
    /// ```
    pub fn action_type(&self) -> &VoiceflowButtonActionType {
        &self.action_type
    }

    /// Returns a reference to the name of the button.
    ///
    /// # Returns
    ///
    /// A reference to the name string.
    ///
    /// # Example
    ///
    /// ```
    /// let name = button.name();
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns a reference to the path of the button.
    ///
    /// # Returns
    ///
    /// A reference to the path string.
    ///
    /// # Example
    ///
    /// ```
    /// let path = button.path();
    /// ```
    pub fn path(&self) -> &String {
        &self.path
    }
}
impl FromValue for VoiceflowButton{
    /// Attempts to convert a JSON `Value` into a `VoiceflowButton` instance.
    ///
    /// # Parameters
    ///
    /// * `value` - A reference to the JSON `Value` to convert from.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the `VoiceflowButton` instance if the conversion
    /// succeeds, or a `VoiceflousionError` if the conversion fails. If the conversion
    /// succeeds but there is no meaningful value, `None` can be returned.
    ///
    /// # Example
    ///
    /// ```
    /// let json_value = serde_json::json!({
    ///     "name": "Button 1",
    ///     "request": {
    ///         "type": "path",
    ///         "payload": {
    ///             "actions": [{
    ///                 "type": "open_url",
    ///                 "payload": { "url": "https://example.com" }
    ///             }]
    ///         }
    ///     }
    /// });
    /// let button = VoiceflowButton::from_value(&json_value)?;
    /// ```
    fn from_value(value: &Value) -> Result<Option<Self>, VoiceflousionError> {
        let name = value.get("name")
            .and_then(|name| name.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button name".to_string(), value.clone())))?
            .to_string();

        let request = value.get("request")
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button request".to_string(), value.clone())))?;

        let path = request.get("type")
            .and_then(|path|  path.as_str())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button path".to_string(), value.clone())))?
            .to_string();


        let actions = request.get("payload")
            .and_then(|payload| payload.get("actions"))
            .and_then(|actions| actions.as_array())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button actions".to_string(), value.clone())))?;

        let action_type = if let Some(action) = actions.iter().find(|action| {
                action.get("type")
                    .and_then(|action_type| action_type.as_str())
                    .map(|action_type| action_type == "open_url")
                    .unwrap_or(false)
            }) {
                let url = action
                    .get("payload")
                    .and_then(|payload| payload.get("url"))
                    .and_then(|url| url.as_str())
                    .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button url".to_string(), value.clone())))?
                    .to_string();
                VoiceflowButtonActionType::OpenUrl(url)
            } else {
                VoiceflowButtonActionType::Path
            };
        Ok(Some(Self::new(name, path, action_type)))
    }
}