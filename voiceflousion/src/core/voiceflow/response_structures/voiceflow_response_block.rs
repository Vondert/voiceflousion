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
    pub fn json(&self) -> &Value {
        &self.json
    }

    /// Returns a reference to the type of the block.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowResponseBlockType`.
    pub fn block_type(&self) -> &VoiceflowResponseBlockType {
        &self.block_type
    }
}