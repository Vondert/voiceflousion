use chrono::Utc;
use serde_json::Value;
use crate::integrations::utils::interaction_type::InteractionType;
use crate::voiceflow::VoiceflousionError;

pub trait Update: Sized + Send + Sync{
    fn chat_id(&self) -> &String;
    fn message_id(&self) -> &String;
    fn interaction_time(&self) -> i64;
    fn interaction_type(&self) -> &InteractionType;
    fn from_request_body(body: Value) -> Result<Self, VoiceflousionError>;
    fn is_deprecated(&self, last_response_time: i64) -> Result<(), VoiceflousionError>{
        if last_response_time > self.interaction_time(){
            return Err(VoiceflousionError::RequestError("Deprecated message".to_string()));
        }
        Ok(())
    }
}