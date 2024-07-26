use std::ops::Deref;
use serde_json::Value;
use crate::core::base_structs::UpdateBase;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// A trait for handling updates in the integration.
///
/// The `Update` trait provides methods for accessing update details,
/// creating updates from request bodies, and checking if an update is deprecated.
pub trait Update: Deref<Target = UpdateBase> + Sized + Send + Sync {

    /// Creates an update from a JSON request body.
    ///
    /// # Parameters
    ///
    /// * `body` - A JSON `Value` representing the request body.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing the `Update` instance or a `VoiceflousionError` if the conversion fails.
    fn from_request_body(body: Value) -> VoiceflousionResult<Self>;

    /// Checks if the update is deprecated based on the last response time.
    ///
    /// # Parameters
    ///
    /// * `last_response_time` - The timestamp of the last response.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` indicating whether the update is deprecated.
    fn is_deprecated(&self, last_response_time: i64) -> VoiceflousionResult<()> {
        if last_response_time > self.interaction_time() {
            return Err(VoiceflousionError::DeprecatedError(self.chat_id().clone(), self.update_id().clone()));
        }
        Ok(())
    }
}