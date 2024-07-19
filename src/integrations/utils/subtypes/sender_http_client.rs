use std::ops::Deref;
use std::time::Duration;
use reqwest::Client;

pub struct SenderHttpClient{
    client: Client
}
impl Deref for SenderHttpClient{
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
impl SenderHttpClient {
    pub fn new (max_sessions_per_moment: usize) ->  Self{
        Self{
            client: Client::builder()
                .pool_max_idle_per_host(max_sessions_per_moment)
                .pool_idle_timeout(Duration::from_secs(60))
                .build().unwrap()
        }
    }
}