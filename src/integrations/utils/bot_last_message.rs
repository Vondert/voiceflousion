use std::marker::PhantomData;
use crate::integrations::utils::traits::Responder;
use crate::voiceflow::VoiceflowBlock;

pub struct BotLastMessage<R: Responder>{
    block: VoiceflowBlock,
    message_id: String,
    _responder: PhantomData<R>
}
impl<R: Responder> BotLastMessage<R>{
    pub fn new(block: VoiceflowBlock, message_id: String) -> Self{
        Self{
            block,
            message_id,
            _responder: Default::default(),
        }
    }
    pub fn from_responder(responder: Option<&R>) -> Option<Self>{
        if let Some(resp) = responder{
            Some(Self::new(
                resp.message_content().clone(),
                resp.message_id().clone()
            ))
        }
        else{
            None
        }
    }
    pub fn id(&self) -> &String{
        &self.message_id
    }
    pub fn block(&self) -> &VoiceflowBlock{
        &self.block
    }
}
