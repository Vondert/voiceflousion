use serde_json::Value;
use crate::voiceflow::response_structures::voiceflow_response_block_type::VoiceflowResponseBlockType;

#[derive(Debug)]
pub(crate) struct VoiceflowResponseBlock{
    block_type: VoiceflowResponseBlockType,
    json: Value
}

impl VoiceflowResponseBlock{
    pub(crate) fn new(block_type: VoiceflowResponseBlockType, json: Value) -> Self{
        Self{
            block_type,
            json
        }
    }
    pub fn json(&self) -> &Value{
        &self.json
    }
    pub fn block_type(&self) -> &VoiceflowResponseBlockType{
        &self.block_type
    }
}