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
    ///
    /// # Example
    ///
    /// ```
    /// struct MyUpdate {
    ///     chat_id: String,
    ///     update_id: String,
    ///     interaction_time: i64,
    ///     interaction_type: InteractionType,
    /// }
    ///
    /// impl Update for MyUpdate {
    ///     fn chat_id(&self) -> &String {
    ///         &self.chat_id
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    ///
    /// let update = MyUpdate {
    ///     chat_id: "chat_id_value".to_string(),
    ///     update_id: "update_id_value".to_string(),
    ///     interaction_time: 1624478392,
    ///     interaction_type: InteractionType::default(),
    /// };
    /// let chat_id = update.chat_id();
    /// ```
    fn chat_id(&self) -> &String;

    /// Returns the update ID.
    ///
    /// # Returns
    ///
    /// A reference to the update ID string.
    ///
    /// # Example
    ///
    /// ```
    /// struct MyUpdate {
    ///     chat_id: String,
    ///     update_id: String,
    ///     interaction_time: i64,
    ///     interaction_type: InteractionType,
    /// }
    ///
    /// impl Update for MyUpdate {
    ///     fn update_id(&self) -> &String {
    ///         &self.update_id
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    ///
    /// let update = MyUpdate {
    ///     chat_id: "chat_id_value".to_string(),
    ///     update_id: "update_id_value".to_string(),
    ///     interaction_time: 1624478392,
    ///     interaction_type: InteractionType::default(),
    /// };
    /// let update_id = update.update_id();
    /// ```
    fn update_id(&self) -> &String;

    /// Returns the interaction time.
    ///
    /// # Returns
    ///
    /// An `i64` representing the interaction time.
    ///
    /// # Example
    ///
    /// ```
    /// struct MyUpdate {
    ///     chat_id: String,
    ///     update_id: String,
    ///     interaction_time: i64,
    ///     interaction_type: InteractionType,
    /// }
    ///
    /// impl Update for MyUpdate {
    ///     fn interaction_time(&self) -> i64 {
    ///         &self.interaction_time
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    ///
    /// let update = MyUpdate {
    ///     chat_id: "chat_id_value".to_string(),
    ///     update_id: "update_id_value".to_string(),
    ///     interaction_time: 1624478392,
    ///     interaction_type: InteractionType::default(),
    /// };
    /// let interaction_time = update.interaction_time();
    /// ```
    fn interaction_time(&self) -> i64;

    /// Returns the type of interaction.
    ///
    /// # Returns
    ///
    /// A reference to the `InteractionType`.
    ///
    /// # Example
    ///
    /// ```
    /// struct MyUpdate {
    ///     chat_id: String,
    ///     update_id: String,
    ///     interaction_time: i64,
    ///     interaction_type: InteractionType,
    /// }
    ///
    /// impl Update for MyUpdate {
    ///     fn interaction_type(&self) -> &InteractionType {
    ///         &self.interaction_type
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    ///
    /// let update = MyUpdate {
    ///     chat_id: "chat_id_value".to_string(),
    ///     update_id: "update_id_value".to_string(),
    ///     interaction_time: 1624478392,
    ///     interaction_type: InteractionType::default(),
    /// };
    /// let interaction_type = update.interaction_type();
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    ///
    /// struct MyUpdate {
    ///     chat_id: String,
    ///     update_id: String,
    ///     interaction_time: i64,
    ///     interaction_type: InteractionType,
    /// }
    ///
    /// impl Update for MyUpdate {
    ///     fn from_request_body(body: Value) -> Result<Self, VoiceflousionError> {
    ///         //Implement conversion here
    ///     }
    ///
    ///     // Other method implementations...
    /// }
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// update.is_deprecated(last_response_time)?;
    /// ```
    fn is_deprecated(&self, last_response_time: i64) -> Result<(), VoiceflousionError> {
        if last_response_time > self.interaction_time() {
            return Err(VoiceflousionError::DeprecatedError(self.chat_id().clone(), self.update_id().clone()));
        }
        Ok(())
    }
}