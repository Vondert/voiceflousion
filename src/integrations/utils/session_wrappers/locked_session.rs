use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::MutexGuard;
use crate::integrations::utils::session_wrappers::Session;
use crate::integrations::utils::subtypes::SentMessage;
use crate::voiceflow::VoiceflousionError;

pub struct LockedSession<'g>{
    session: &'g Arc<Session>,
    _guard:  MutexGuard<'g, bool>
}
impl<'g> Deref for LockedSession<'g>{
    type Target = Arc<Session>;

    fn deref(&self) -> &'g Self::Target {
        self.session
    }
}
impl<'g> LockedSession<'g>{
    pub fn try_from_session(session: &'g Arc<Session>) -> Result<Self, VoiceflousionError>{
        let binding = &session.lock;
        let guard = binding.try_lock().map_err(|_| VoiceflousionError::SessionLockError(session.get_cloned_chat_id()))?;

        Ok(Self{
            session,
            _guard: guard
        })
    }
    pub async fn set_previous_message(&self, message: Option<SentMessage>) -> () {
        let mut write = self.session.write_previous_message().await;
        *write = message;
    }
    pub fn set_last_interaction(&self, last_interaction: Option<i64>) -> (){
        self.last_interaction.store(last_interaction, Ordering::SeqCst)
    }
}