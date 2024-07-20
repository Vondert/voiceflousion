use crate::voiceflow::VoiceflowBlock;

pub struct SentMessage{
    block: VoiceflowBlock,
    message_id: String,
    date: i64
}
impl SentMessage{
    pub fn new(block: VoiceflowBlock, message_id: String, date: i64) -> Self{
        Self{
            block,
            message_id,
            date
        }
    }
    pub fn id(&self) -> &String{
        &self.message_id
    }
    pub fn block(&self) -> &VoiceflowBlock{
        &self.block
    }
    pub fn date(&self) -> i64{
        self.date
    }
}