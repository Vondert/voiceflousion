use serde_json::{Map, Value};
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

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

    payload: Value
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
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowButton;
    ///
    /// let button = VoiceflowButton::new("Button 1".to_string(), "/path".to_string(), VoiceflowButtonActionType::Path, Value::Null);
    /// ```
    pub fn new(name: String, path: String, action_type: VoiceflowButtonActionType, payload: Value) -> Self {
        Self {
            name,
            path,
            action_type,
            payload
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
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowButton;
    ///
    /// let button = VoiceflowButton::new("Button 1".to_string(), "/path".to_string(), VoiceflowButtonActionType::Path, Value::Null);
    ///
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
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowButton;
    ///
    /// let button = VoiceflowButton::new("Button 1".to_string(), "/path".to_string(), VoiceflowButtonActionType::Path, Value::Null);
    ///
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
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowButton;
    ///
    /// let button = VoiceflowButton::new("Button 1".to_string(), "/path".to_string(), VoiceflowButtonActionType::Path, Value::Null);
    ///
    /// let path = button.path();
    /// ```
    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn payload(&self) -> &Value{
        &self.payload
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
    fn from_value(value: &Value) -> VoiceflousionResult<Option<Self>> {
        println!("\nButton {:?}\n", &value);
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

        let request_payload = request.get("payload")
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button payload".to_string(), value.clone())))?;

        let payload = extract_values_except_actions(request_payload);

        let option_actions = match request_payload.get("actions"){
            Some(actions) => {
                Some(actions.as_array().ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button actions".to_string(), value.clone())))?)
            },
            None =>{
               None
            }
        };

        let mut action_type = VoiceflowButtonActionType::Path;

        if let Some(actions) = option_actions{
            if let Some(action) = actions.iter().find(|action| {
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

                action_type = VoiceflowButtonActionType::OpenUrl(url)
            }
        }

        Ok(Some(Self::new(name, path, action_type, payload)))
    }
}

fn extract_values_except_actions(payload: &Value) -> Value {
    if let Some(obj) = payload.as_object() {
        let mut new_obj = Map::new();

        for (key, value) in obj {
            if key != "actions" {
                new_obj.insert(key.clone(), value.clone());
            }
        }

        Value::Object(new_obj)
    } else {
        Value::Null
    }
}