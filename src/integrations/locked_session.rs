use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::MutexGuard;
use crate::integrations::Session;
use crate::voiceflow::VoiceflowError;

pub struct LockedSession<'g, S: Session>{
    session: &'g Arc<S>,
    _guard:  MutexGuard<'g, bool>
}
impl<'g, S: Session> Deref for LockedSession<'g, S>{
    type Target = Arc<S>;

    fn deref(&self) -> &'g Self::Target {
        self.session
    }
}
impl<'g, S: Session> LockedSession<'g, S>{
    pub fn try_from_session(session: &'g Arc<S>) -> Result<Self, VoiceflowError>{
        let guard = session.try_lock_sync()?;
        Ok(Self{
            session,
            _guard: guard
        })
    }
    pub fn session(&self) -> Arc<S>{
        self.session.clone()
    }
}