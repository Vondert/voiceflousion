use crate::integrations::TelegramSession;
use crate::voiceflow::{VoiceflowClient, VoiceflowError};
use crate::voiceflow::dialog_blocks::VoiceflowMessage;
use crate::voiceflow::request_structures::State;

pub(crate) struct TelegramClient<'a>{
    bot_id: String,
    bot_token: String,
    voiceflow_client: &'a VoiceflowClient
}

impl<'a> TelegramClient<'a> {
    pub fn new(bot_token: String, voiceflow_client: &'a VoiceflowClient) -> Self{
        let bot_id = bot_token.split(':').next().unwrap().to_string();
        Self{
            bot_id,
            bot_token,
            voiceflow_client
        }
    }
    pub async fn launch_voiceflow_dialog(&self, telegram_session: &TelegramSession, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>{
        self.voiceflow_client.launch_dialog(&*telegram_session, state).await
    }
}