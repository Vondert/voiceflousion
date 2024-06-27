use crate::voiceflow::request_structures::{ActionType, Session, State};

#[derive(Debug)]
pub(crate) struct VoiceflowRequestBody{
    action: ActionType,
    session: Option<Session>,
    state: Option<State>
}

impl VoiceflowRequestBody{
    pub fn to_json(&self) -> String{
        let mut json_string = format!("{{\"action\":{{\"type\":\"{}\"}}", &self.action.to_string());

        if let Some(session) = &self.session {
            let session_json = serde_json::to_string(session).unwrap();
            json_string.push_str(&format!(", \"session\":{}", session_json));
        }

        if let Some(state) = &self.state {
            let state_json = serde_json::to_string(state).unwrap();
            json_string.push_str(&format!(", \"state\":{}", state_json));
        }
        json_string.push('}');
        json_string
    }
}


pub(crate) struct VoiceflowRequestBodyBuilder{
    action: ActionType,
    session: Option<Session>,
    state: Option<State>
}
impl VoiceflowRequestBodyBuilder{
    pub fn new (action: ActionType) -> Self{
        Self{
            action,
            session: None,
            state: None,
        }
    }
    pub fn session(mut self, session: Option<Session>) -> Self{
        self.session = session;
        self
    }
    pub fn state(mut self, state: Option<State>) -> Self{
        self.state = state;
        self
    }
    pub fn build (self) -> VoiceflowRequestBody{
        VoiceflowRequestBody{
            action: self.action,
            session: self.session,
            state: self.state,
        }
    }
}
