use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use serde::Deserialize;
use crate::core::subtypes::BotAuthToken;

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    #[serde(flatten)]
    params: HashMap<String, String>,
}
impl Deref for QueryParams{
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

impl DerefMut for QueryParams{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.params
    }
}

impl QueryParams{
    pub fn extract_bot_auth_token(&mut self) -> Option<BotAuthToken>{
        if let Some(token) = self.remove("voiceflousion_bot_token"){
            Some(BotAuthToken::new(token))
        }
        else{
            None
        }
    }
}