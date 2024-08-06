use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use reqwest::Response;
use crate::core::base_structs::ResponderBase;
use crate::core::traits::Responder;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::VoiceflousionResult;

#[derive(Debug)]
pub struct DiscordResponder{
    /// The base structure that provides core functionalities.
    responder_base: ResponderBase
}

impl Deref<Target=ResponderBase> for DiscordResponder {
    type Target = ResponderBase;

    fn deref(&self) -> &Self::Target {
        &self.responder_base
    }
}
impl Responder for DiscordResponder{
    async fn from_response(response: Response, content: VoiceflowBlock) -> VoiceflousionResult<Self> {
        todo!()
    }
}