/// Represents the type of action for a Voiceflow button.
///
/// `VoiceflowButtonActionType` is an enum that can represent different types of actions
/// such as opening a URL or following a path, within the Voiceflow dialog.
#[derive(Debug, Clone)]
pub enum VoiceflowButtonActionType {
    /// An action to open a URL.
    OpenUrl(String),
    /// An action to follow a path within the Voiceflow dialog.
    Path
}

impl VoiceflowButtonActionType{
    pub fn is_url(&self) -> bool{
        match &self {
            VoiceflowButtonActionType::OpenUrl(_) => true,
            VoiceflowButtonActionType::Path => false
        }
    }
}