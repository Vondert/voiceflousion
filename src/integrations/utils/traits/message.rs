use crate::voiceflow::VoiceflowMessage;

pub trait Message: Send + Sync{
    fn from_voiceflow_message(voiceflow_message: VoiceflowMessage) -> Self;
}