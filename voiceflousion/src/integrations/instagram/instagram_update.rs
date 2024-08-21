use std::fmt::Debug;
use std::ops::Deref;
use serde_json::Value;
use crate::core::base_structs::UpdateBase;
use crate::core::traits::Update;
use crate::errors::VoiceflousionResult;

#[derive(Debug)]
pub struct InstagramUpdate{
    update_base: UpdateBase
}

impl InstagramUpdate{
    fn new(update_base: UpdateBase) -> Self{
        Self{
            update_base
        }
    }
}

impl Deref for InstagramUpdate{
    type Target = UpdateBase;

    fn deref(&self) -> &Self::Target {
        &self.update_base
    }
}

impl Update for InstagramUpdate{
    fn from_request_body(body: Value) -> VoiceflousionResult<Self> {
        todo!()
    }
}