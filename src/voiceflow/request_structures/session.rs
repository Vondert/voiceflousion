use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Session{
    #[serde(rename = "sessionID")]
    session_id: String,
    #[serde(rename = "userID")]
    user_id: String,
}

impl Session{
    pub fn new(session_id: String, user_id: String) -> Self{
        Self{
          session_id,
          user_id
        }
    }
}