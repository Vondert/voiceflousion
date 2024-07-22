use serde_json::Value;
use crate::core::voiceflow::response_structures::voiceflow_response_block_type::VoiceflowResponseBlockType;

/// Represents a block in a Voiceflow response.
///
/// `VoiceflowResponseBlock` contains the type of block and the associated JSON data.
#[derive(Debug)]
pub(crate) struct VoiceflowResponseBlock {
    /// The type of the Voiceflow response block.
    block_type: VoiceflowResponseBlockType,

    /// The JSON data associated with the block.
    json: Value,
}

impl VoiceflowResponseBlock{
    /// Creates a new `VoiceflowResponseBlock`.
    ///
    /// # Parameters
    ///
    /// * `block_type` - The type of the Voiceflow response block.
    /// * `json` - The JSON data associated with the block.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowResponseBlock`.
    ///
    /// # Example
    ///
    /// ```
    /// let block_type = VoiceflowResponseBlockType::Text;
    /// let json = serde_json::json!({"text": "Hello, world!"});
    /// let response_block = VoiceflowResponseBlock::new(block_type, json);
    /// ```
    pub(crate) fn new(block_type: VoiceflowResponseBlockType, json: Value) -> Self {
        Self {
            block_type,
            json,
        }
    }

    /// Returns a reference to the JSON data of the block.
    ///
    /// # Returns
    ///
    /// A reference to the JSON data.
    ///
    /// # Example
    ///
    /// ```
    /// let json = response_block.json();
    /// ```
    pub fn json(&self) -> &Value {
        &self.json
    }

    /// Returns a reference to the type of the block.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowResponseBlockType`.
    ///
    /// # Example
    ///
    /// ```
    /// let block_type = response_block.block_type();
    /// ```
    pub fn block_type(&self) -> &VoiceflowResponseBlockType {
        &self.block_type
    }
}