use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::MutexGuard;
use crate::integrations::utils::sent_message::SentMessage;
use crate::integrations::utils::SessionWrapper;
use crate::integrations::utils::traits::Session;
use crate::voiceflow::VoiceflousionError;

pub struct LockedSession<'g, S: Session>{
    session: &'g Arc<SessionWrapper<S>>,
    _guard:  MutexGuard<'g, bool>
}
impl<'g, S: Session> Deref for LockedSession<'g, S>{
    type Target = Arc<SessionWrapper<S>>;

    fn deref(&self) -> &'g Self::Target {
        self.session
    }
}
impl<'g, S: Session> LockedSession<'g, S>{
    pub fn try_from_session(session: &'g Arc<SessionWrapper<S>>) -> Result<Self, VoiceflousionError>{
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
    pub async fn set_previous_message(&self, message: Option<SentMessage>) -> () {
        let mut write = self.session.write_previous_message().await;
        *write = message;
    }
    pub async fn set_last_interaction(&self, interaction: i64) -> (){
        let mut write = self.write_last_interaction().await;
        *write = Some(interaction);
    }
    pub fn session(&self) -> Arc<SessionWrapper<S>>{
        self.session.clone()
    }
}