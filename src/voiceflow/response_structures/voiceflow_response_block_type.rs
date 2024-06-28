#[derive(Debug)]
pub(crate) enum VoiceflowResponseBlockType {
    Text,
    Choice,
    Block,
    CardV2,
    Visual,
    Carousel,
    None
}
impl VoiceflowResponseBlockType {
    pub fn new(response_type: &str) -> VoiceflowResponseBlockType {
        match response_type {
            "text" => VoiceflowResponseBlockType::Text,
            "choice" => VoiceflowResponseBlockType::Choice,
            "block" => VoiceflowResponseBlockType::Block,
            "cardV2" => VoiceflowResponseBlockType::CardV2,
            "visual" => VoiceflowResponseBlockType::Visual,
            "carousel" => VoiceflowResponseBlockType::Carousel,
            _ => VoiceflowResponseBlockType::None
        }
    }
}