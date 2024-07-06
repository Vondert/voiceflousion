use crate::integrations::utils::traits::Message;
use crate::voiceflow::VoiceflowMessage;
#[derive(Debug)]
pub struct TelegramMessage{

}
impl Message for TelegramMessage{
    fn from_voiceflow_message(voiceflow_message: VoiceflowMessage) -> Self{
        todo!()
    }
}