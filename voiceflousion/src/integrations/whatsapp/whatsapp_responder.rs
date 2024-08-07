use std::fmt::Debug;
use std::ops::Deref;
use async_trait::async_trait;
use reqwest::Response;
use crate::core::base_structs::ResponderBase;
use crate::core::traits::Responder;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::VoiceflousionResult;

#[derive(Debug)]
pub struct WhatsAppResponder{
    /// The base structure that provides core functionalities.
    responder_base: ResponderBase
}

impl Deref for WhatsAppResponder {
    type Target = ResponderBase;

    fn deref(&self) -> &Self::Target {
        &self.responder_base
    }
}

#[async_trait]
impl Responder for WhatsAppResponder{
    async fn from_response(response: Response, content: VoiceflowBlock) -> VoiceflousionResult<Self> {
        todo!()
    }
}