use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::VoiceflowError;
#[derive(Debug)]
pub(super) struct VoiceflowImage{
    url: String,
    height: u32,
    width: u32
}
impl VoiceflowImage{
    pub fn new(url: String, height: u32, width: u32) -> Self{
        Self{
            url,
            height,
            width
        }
    }
}
impl VoiceflowBlock for VoiceflowImage {}

impl FromValue for VoiceflowImage{
    type Error = VoiceflowError;
    fn from_value(value: &Value) -> Result<Self, Self::Error> {
        let payload = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Image".to_string(), value.clone())))?;

        let height = payload.get("dimensions")
            .and_then(|dimensions| dimensions.get("height"))
            .and_then(|height| height.as_u64())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Image".to_string(), value.clone())))? as u32;

        let width = payload.get("dimensions")
            .and_then(|dimensions| dimensions.get("width"))
            .and_then(|width| width.as_u64())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Image".to_string(), value.clone())))? as u32;

        let url = payload.get("image")
            .and_then(|image| image.as_str())
            .ok_or_else(|| VoiceflowError::BlockConvertationError(("Image".to_string(), value.clone())))?
            .to_string();

        Ok(Self::new(url, height, width))
    }
}