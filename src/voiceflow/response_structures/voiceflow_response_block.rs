use serde_json::Value;
use crate::voiceflow::response_structures::voiceflow_response_block_type::VoiceflowResponseBlockType;

#[derive(Debug)]
pub(crate) struct VoiceflowResponseBlock{
    pub block_type: VoiceflowResponseBlockType,
    pub json: Value
}

impl VoiceflowResponseBlock{
    pub(crate) fn new(block_type: VoiceflowResponseBlockType, json: Value) -> Self{
        Self{
            block_type,
            json
        }
    }
}