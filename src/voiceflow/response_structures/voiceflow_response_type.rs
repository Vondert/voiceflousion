pub(super) enum VoiceflowResponseType{
    Text,
    Choice,
    Block,
    None
}
impl VoiceflowResponseType{
    pub fn new(response_type: &str) -> VoiceflowResponseType{
        match response_type {
            "text" => VoiceflowResponseType::Text,
            "choice" => VoiceflowResponseType::Choice,
            "block" => VoiceflowResponseType::Block,
            _ => VoiceflowResponseType::None
        }
    }
}