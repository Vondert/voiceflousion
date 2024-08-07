use std::ops::Deref;
use serde_json::Value;
use crate::core::base_structs::UpdateBase;
use crate::core::traits::Update;
use crate::errors::VoiceflousionResult;

#[derive(Debug)]
pub struct WhatsAppUpdate{
    /// The base structure that provides core functionalities.
    update_base: UpdateBase,
}

impl Deref for WhatsAppUpdate {
    type Target = UpdateBase;

    fn deref(&self) -> &Self::Target {
        &self.update_base
    }
}

impl Update for WhatsAppUpdate{
    fn from_request_body(body: Value) -> VoiceflousionResult<Self> {
        todo!()
    }
}