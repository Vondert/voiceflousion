use serde::Serialize;
use crate::voiceflow::request_structures::{VoiceflowSession, State};
use crate::voiceflow::request_structures::action::Action;

#[derive(Debug, Serialize)]
pub(crate) struct VoiceflowRequestBody<'a> {
    action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<&'a VoiceflowSession>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<State>,
}

impl<'a> VoiceflowRequestBody<'a> {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub(crate) struct VoiceflowRequestBodyBuilder<'a> {
    action: Action,
    session: Option<&'a VoiceflowSession>,
    state: Option<State>,
}

impl<'a> VoiceflowRequestBodyBuilder<'a> {
    pub fn new(action: Action) -> Self {
        Self {
            action,
            session: None,
            state: None,
        }
    }

    pub fn session(mut self, session: Option<&'a VoiceflowSession>) -> Self {
        self.session = session;
        self
    }

    pub fn state(mut self, state: Option<State>) -> Self {
        self.state = state;
        self
    }

    pub fn build(self) -> VoiceflowRequestBody<'a> {
        VoiceflowRequestBody {
            action: self.action,
            session: self.session,
            state: self.state,
        }
    }
}