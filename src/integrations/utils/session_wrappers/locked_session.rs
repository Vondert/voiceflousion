use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::MutexGuard;
use crate::integrations::utils::bot_last_message::BotLastMessage;
use crate::integrations::utils::SessionWrapper;
use crate::integrations::utils::traits::{Responder, Session};
use crate::voiceflow::{VoiceflousionError, VoiceflowMessage};

pub struct LockedSession<'g, S: Session, R: Responder>{
    session: &'g Arc<SessionWrapper<S, R>>,
    _guard:  MutexGuard<'g, bool>
}
impl<'g, S: Session, R: Responder> Deref for LockedSession<'g, S, R>{
    type Target = Arc<SessionWrapper<S, R>>;

    fn deref(&self) -> &'g Self::Target {
        self.session
    }
}
impl<'g, S: Session, R: Responder> LockedSession<'g, S, R>{
    pub fn try_from_session(session: &'g Arc<SessionWrapper<S, R>>) -> Result<Self, VoiceflousionError>{
        let guard = session.try_lock_sync()?;
        Ok(Self{
            session,
            _guard: guard
        })
    }
    /*pub async fn set_previous_message(&self, mut message: VoiceflowMessage, message_id: String) -> () {
        let mut write = self.session.write_previous_message().await;
        let last_message = BotLastMessage::new(message.pop(), message_id);
        *write = message.pop();
    }*/
    pub async fn set_previous_message(&self, message: Option<BotLastMessage<R>>) -> () {
        let mut write = self.session.write_previous_message().await;
        *write = message;
    }
    pub async fn set_last_interaction(&self, interaction: i64) -> (){
        let mut write = self.write_last_interaction().await;
        *write = Some(interaction);
    }
    pub fn session(&self) -> Arc<SessionWrapper<S, R>>{
        self.session.clone()
    }
}