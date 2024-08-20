use std::convert::Infallible;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{HeaderMap, HeaderValue};
use axum::http::request::Parts;

pub struct VoiceflousionHeadersWrapper{
    headers: HeaderMap
}
impl VoiceflousionHeadersWrapper{
    fn new(headers: HeaderMap) -> Self{
        Self{
            headers
        }
    }
    pub fn get_header_map(&self) -> &HeaderMap {
        &self.headers
    }
    pub fn get_header(&self, header_name: &str) -> Option<&HeaderValue>{
        self.headers.get(header_name)
    }
    pub fn get_header_str(&self, header_name: &str) -> Option<&str>{
        self.get_header(header_name).and_then(|header|{
            header.to_str().ok()
        })
    }

    pub fn get_header_str_or_empty(&self, header_name: &str) -> &str{
        self.get_header_str(header_name).unwrap_or_default()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for VoiceflousionHeadersWrapper
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = parts.headers.clone();
        Ok(VoiceflousionHeadersWrapper::new(headers))
    }
}