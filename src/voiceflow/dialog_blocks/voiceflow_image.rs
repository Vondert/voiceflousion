use serde_json::Value;
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::VoiceflousionError;
#[derive(Debug)]
pub struct VoiceflowImage{
    url: String,
    height: Option<u64>,
    width: Option<u64>
}
impl VoiceflowImage{
    pub fn new(url: String, height: Option<u64>, width: Option<u64>) -> Self{
        Self{
            url,
            height,
            width
        }
    }
    pub fn url(&self) -> &String{
        &self.url
    }
    pub fn height(&self) -> Option<u64>{
        self.height
    }
    pub fn width(&self) -> Option<u64>{
        self.width
    }
}

impl FromValue for VoiceflowImage{
    type Error = VoiceflousionError;
    fn from_value(value: &Value) -> Result<Option<Self>, Self::Error> {
        let payload = value.get("trace")
            .and_then(|trace| trace.get("payload"))
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Image".to_string(), value.clone())))?;

        let url = payload.get("image")
            .and_then(|image| image.as_str())
            .ok_or_else(|| VoiceflousionError::BlockConvertationError(("Image".to_string(), value.clone())))?
            .to_string();

        let height = payload.get("dimensions")
            .and_then(|dimensions| dimensions.get("height"))
            .and_then(|height| height.as_u64());

        let width = payload.get("dimensions")
            .and_then(|dimensions| dimensions.get("width"))
            .and_then(|width| width.as_u64());
        if url.is_empty() {
            return Ok(None)
        }
        Ok(Some(Self::new(url, height, width)))
    }
}