use serde_json::{json, Map, Value};
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::core::voiceflow::dialog_blocks::VoiceflowText;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a button in a Voiceflow dialog.
///
/// `VoiceflowButton` contains the name, path, action type, and payload of the button.
#[derive(Debug, Clone)]
pub struct VoiceflowButton {
    /// The action type of the button.
    action_type: VoiceflowButtonActionType,
    /// The name of the button.
    name: String,
    /// The payload associated with the button.
    payload: Value
}

impl VoiceflowButton {

    /// Creates a new `VoiceflowButton` instance.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the button.
    /// * `path` - The path associated with the button.
    /// * `action_type` - The action type of the button.
    /// * `payload` - The payload associated with the button.
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
    /// let button = VoiceflowButton::new("Button 1".to_string(), VoiceflowButtonActionType::Path, Value::Null);
    /// ```
    pub fn new(name: String,payload: Value, option_url: Option<String>) -> Self {
        let action_type = if let Some(url) = option_url{
            VoiceflowButtonActionType::Url(url)
        }
        else{
            VoiceflowButtonActionType::Path
        };
        Self {
            name,
            action_type,
            payload
        }
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
    /// let button = VoiceflowButton::new("Button 1".to_string(), VoiceflowButtonActionType::Path, Value::Null);
    ///
    /// let name = button.name();
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn payload(&self) -> &Value{
        &self.payload
    }
    pub fn get_url_text(&self) -> Option<VoiceflowText>{
        match &self.action_type{
            VoiceflowButtonActionType::Url(url) => {
                let url_text = VoiceflowText::new(url.clone());
                Some(url_text)
            },
            _ => None
        }
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

        let mut payload = extract_payload(request_payload);

        payload.as_object_mut().unwrap().insert("path".to_string(), json!(path));

        let mut option_url = None;

        if let Some(actions_value) = request_payload.get("actions"){
            let actions = actions_value.as_array().ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button actions".to_string(), value.clone())))?;

            if let Some(action) = actions.iter().find(|action| {
                action.get("type")
                    .and_then(|action_type| action_type.as_str())
                    .map(|action_type| action_type == "open_url")
                    .unwrap_or(false)
            }) {
                option_url = Some(action["payload"].get("url").and_then(|url| url.as_str())
                    .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowButton button url".to_string(), value.clone())))?
                    .to_string());
            }
        }

        Ok(Some(Self::new(name, payload, option_url)))
    }
}

/// Extracts the payload from the JSON `Value` by removing specific keys.
///
/// # Parameters
///
/// * `payload` - A reference to the JSON `Value` containing the payload.
///
/// # Returns
///
/// A new JSON `Value` with the specified keys removed.
fn extract_payload(payload: &Value) -> Value {
    if let Some(obj) = payload.as_object() {
        let mut new_obj = Map::new();

        for (key, value) in obj {
            if key != "actions" && key != "label"{
                new_obj.insert(key.clone(), value.clone());
            }
        }

        Value::Object(new_obj)
    } else {
        Value::Null
    }
}