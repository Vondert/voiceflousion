use serde_json::Value;
use crate::core::voiceflow::VoiceflousionError;

/// A trait for converting from a JSON `Value` to an instance of a type.
///
/// The `FromValue` trait defines a method for attempting to convert a JSON `Value`
/// into an instance of a type. The conversion can potentially fail, in which case
/// a `VoiceflousionError` is returned.
pub(crate) trait FromValue: Sized {
    /// Attempts to convert a JSON `Value` into an instance of the implementing type.
    ///
    /// # Parameters
    ///
    /// * `value` - A reference to the JSON `Value` to convert from.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the instance of the type if the conversion
    /// succeeds, or a `VoiceflousionError` if the conversion fails. If the conversion
    /// succeeds but there is no meaningful value, `None` can be returned.
    fn from_value(value: &Value) -> Result<Option<Self>, VoiceflousionError>;
}