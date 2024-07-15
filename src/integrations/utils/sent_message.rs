use crate::voiceflow::VoiceflowBlock;

pub struct SentMessage{
    block: VoiceflowBlock,
    message_id: String
}
impl SentMessage{
    pub fn new(block: VoiceflowBlock, message_id: String) -> Self{
        Self{
            block,
            message_id
        }
    }
    pub fn id(&self) -> &String{
        &self.message_id
    }
    pub fn block(&self) -> &VoiceflowBlock{
        &self.block
    }
}
