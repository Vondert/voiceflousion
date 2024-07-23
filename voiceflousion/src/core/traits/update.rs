use serde_json::Value;
use crate::core::subtypes::InteractionType;
use crate::core::voiceflow::VoiceflousionError;

/// A trait for handling updates in the integration.
///
/// The `Update` trait provides methods for accessing update details,
/// creating updates from request bodies, and checking if an update is deprecated.
pub trait Update: Sized + Send + Sync {
    /// Returns the chat ID associated with the update.
    ///
    /// # Returns
    ///
    /// A reference to the chat ID string.
    fn chat_id(&self) -> &String;

    /// Returns the update ID.
    ///
    /// # Returns
    ///
    /// A reference to the update ID string.
    fn update_id(&self) -> &String;

    /// Returns the interaction time.
    ///
    /// # Returns
    ///
    /// An `i64` representing the interaction time.
    fn interaction_time(&self) -> i64;

    /// Returns the type of interaction.
    ///
    /// # Returns
    ///
    /// A reference to the `InteractionType`.
    fn interaction_type(&self) -> &InteractionType;

    /// Creates an update from a JSON request body.
    ///
    /// # Parameters
    ///
    /// * `body` - A JSON `Value` representing the request body.
    ///
    /// # Returns
    ///
    /// A `Result` containing the update or a `VoiceflousionError` if the conversion fails.
    fn from_request_body(body: Value) -> Result<Self, VoiceflousionError>;

    /// Checks if the update is deprecated based on the last response time.
    ///
    /// # Parameters
    ///
    /// * `last_response_time` - The timestamp of the last response.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the update is deprecated.
    fn is_deprecated(&self, last_response_time: i64) -> Result<(), VoiceflousionError> {
        if last_response_time > self.interaction_time() {
            return Err(VoiceflousionError::DeprecatedError(self.chat_id().clone(), self.update_id().clone()));
        }
        Ok(())
    }
}