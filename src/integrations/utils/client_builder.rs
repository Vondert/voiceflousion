use std::sync::Arc;
use crate::integrations::utils::session_wrappers::Session;
use crate::voiceflow::VoiceflowClient;

pub struct ClientBuilder{
    client_id: String,
    api_key: String,
    voiceflow_client: Arc<VoiceflowClient>,
    sessions: Option<Vec<Session>>,
    max_connections_per_moment: usize,
    is_cleaning: bool,
    session_duration: Option<i64>,
    sessions_cleanup_interval: Option<u64>
}

impl ClientBuilder{
    pub fn new(client_id: String, api_key: String, voiceflow_client: Arc<VoiceflowClient>, max_connections_per_moment: usize) -> Self{
        Self{
            client_id,
            api_key,
            voiceflow_client,
            sessions: None,
            max_connections_per_moment,
            is_cleaning: false,
            session_duration: None,
            sessions_cleanup_interval: None,
        }
    }
    pub fn add_sessions(mut self, sessions: Vec<Session>) -> Self{
        self.sessions = Some(sessions);
        self
    }
    pub fn allow_sessions_cleaning(mut self) -> Self{
        self.is_cleaning = true;
        self
    }
    pub fn add_session_duration(mut self, duration: i64) -> Self{
        self.session_duration = Some(duration);
        self
    }
    pub fn add_cleaning_interval(mut self, interval: u64) -> Self{
        self.sessions_cleanup_interval = Some(interval);
        self
    }

    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    pub fn voiceflow_client(&self) -> &Arc<VoiceflowClient> {
        &self.voiceflow_client
    }

    pub fn sessions(self) -> Option<Vec<Session>> {
        self.sessions
    }

    pub fn max_connections_per_moment(&self) -> usize {
        self.max_connections_per_moment
    }

    pub fn is_cleaning(&self) -> bool {
        self.is_cleaning
    }

    pub fn session_duration(&self) -> Option<i64> {
        self.session_duration
    }

    pub fn sessions_cleanup_interval(&self) -> Option<u64> {
        self.sessions_cleanup_interval
    }
}