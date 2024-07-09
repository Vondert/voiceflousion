use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::MutexGuard;
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
    pub fn session(&self) -> Arc<SessionWrapper<S>>{
        self.session.clone()
    }
}