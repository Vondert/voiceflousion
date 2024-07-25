use crate::core::subtypes::InteractionType;

/// `UpdateBase` is the foundational struct for managing updates within the system.
///
/// This struct encapsulates essential components such as the chat ID, interaction time,
/// type of interaction, and the update ID.
#[derive(Debug)]
pub struct UpdateBase{
    /// The chat ID associated with the update.
    chat_id: String,
    /// The interaction time of the update.
    interaction_time: i64,
    /// The type of interaction.
    interaction_type: InteractionType,
    /// The update ID.
    update_id: String,
}
impl UpdateBase{
    /// Creates a new `UpdateBase`.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID associated with the update.
    /// * `interaction_time` - The interaction time of the update.
    /// * `interaction_type` - The type of interaction.
    /// * `update_id` - The update ID.
    ///
    /// # Returns
    ///
    /// A new instance of `UpdateBase`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::UpdateBase;
    /// use voiceflousion::core::subtypes::InteractionType;
    ///
    /// let interaction_type = InteractionType::new("message".to_string(), Some("path".to_string()));
    /// let update_base = UpdateBase::new("chat_id".to_string(), 1627554661, interaction_type, "update_id".to_string());
    /// ```
    pub fn new(chat_id: String, interaction_time: i64, interaction_type: InteractionType, update_id: String) -> Self {
        Self {
            chat_id,
            interaction_time,
            interaction_type,
            update_id,
        }
    }

    /// Returns the chat ID associated with the update.
    ///
    /// # Returns
    ///
    /// A reference to the chat ID string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::UpdateBase;
    /// use voiceflousion::core::subtypes::InteractionType;
    ///
    /// let interaction_type = InteractionType::new("message".to_string(), Some("path".to_string()));
    /// let update_base = UpdateBase::new("chat_id".to_string(), 1627554661, interaction_type, "update_id".to_string());
    /// let chat_id = update_base.chat_id();
    /// ```
    pub fn chat_id(&self) -> &String {
        &self.chat_id
    }

    /// Returns the update ID.
    ///
    /// # Returns
    ///
    /// A reference to the update ID string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::UpdateBase;
    /// use voiceflousion::core::subtypes::InteractionType;
    ///
    /// let interaction_type = InteractionType::new("message".to_string(), Some("path".to_string()));
    /// let update_base = UpdateBase::new("chat_id".to_string(), 1627554661, interaction_type, "update_id".to_string());
    /// let update_id = update_base.update_id();
    /// ```
    pub fn update_id(&self) -> &String {
        &self.update_id
    }

    /// Returns the interaction time.
    ///
    /// # Returns
    ///
    /// An `i64` representing the interaction time.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::UpdateBase;
    /// use voiceflousion::core::subtypes::InteractionType;
    ///
    /// let interaction_type = InteractionType::new("message".to_string(), Some("path".to_string()));
    /// let update_base = UpdateBase::new("chat_id".to_string(), 1627554661, interaction_type, "update_id".to_string());
    /// let interaction_time = update_base.interaction_time();
    /// ```
    pub fn interaction_time(&self) -> i64 {
        self.interaction_time
    }

    /// Returns the type of interaction.
    ///
    /// # Returns
    ///
    /// A reference to the `InteractionType`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::UpdateBase;
    /// use voiceflousion::core::subtypes::InteractionType;
    ///
    /// let interaction_type = InteractionType::new("message".to_string(), Some("path".to_string()));
    /// let update_base = UpdateBase::new("chat_id".to_string(), 1627554661, interaction_type, "update_id".to_string());
    /// let interaction_type = update_base.interaction_type();
    /// ```
    pub fn interaction_type(&self) -> &InteractionType {
        &self.interaction_type
    }
}