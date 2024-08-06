use serde_json::Value;
use crate::core::base_structs::ClientBase;
use crate::core::session_wrappers::LockedSession;
use crate::core::traits::{Client, Sender};
use crate::core::voiceflow::State;
use crate::errors::VoiceflousionResult;

pub struct DiscordClient{

}

impl Client for DiscordClient{
    const ORIGINS: &'static [&'static str] = &[];
    type ClientUpdate<'async_trait> = ();
    type ClientSender<'async_trait>
    where
        Self: 'async_trait
    = ();

    fn client_base(&self) -> &ClientBase<Self::ClientSender<'_>> {
        todo!()
    }

    async fn handle_button_interaction(&self, locked_session: &LockedSession<'_>, interaction_time: i64, button_path: &String, update_state: Option<State>, update: &Self::ClientUpdate<'_>, payload: &Value) -> VoiceflousionResult<Vec<crate::core::traits::sender::SenderResponder>> {
        todo!()
    }
}