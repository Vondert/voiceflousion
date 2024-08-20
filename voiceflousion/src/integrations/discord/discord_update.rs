use std::ops::Deref;
use serde_json::Value;
use crate::core::base_structs::UpdateBase;
use crate::core::traits::Update;
use crate::errors::VoiceflousionResult;

#[derive(Debug)]
pub struct DiscordUpdate{
    update_base: UpdateBase
}

impl Deref for DiscordUpdate {
    type Target = UpdateBase;

    fn deref(&self) -> &Self::Target {
        &self.update_base
    }
}

impl Update for DiscordUpdate{
    fn from_request_body(body: Value) -> VoiceflousionResult<Self> {
        todo!()
    }
}