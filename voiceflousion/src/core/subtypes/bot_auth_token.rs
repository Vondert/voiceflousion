use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BotAuthToken{
    token: String
}
impl BotAuthToken{
    pub(crate) fn new(token: String) -> Self{
        Self{
            token
        }
    }

    pub fn token(&self) -> &String{
        &self.token
    }
}