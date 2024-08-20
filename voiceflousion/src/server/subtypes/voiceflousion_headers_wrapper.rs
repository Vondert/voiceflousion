use std::convert::Infallible;
use std::ops::{Deref, DerefMut};
use async_trait::async_trait;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::HeaderMap;
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
}

#[async_trait]
impl<S> FromRequest<S> for VoiceflousionHeadersWrapper
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        HeaderMap::from_request(req, state).await.map(|headers| VoiceflousionHeadersWrapper::new(headers))
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
impl Deref for VoiceflousionHeadersWrapper{
    type Target = HeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.headers
    }
}

impl DerefMut for VoiceflousionHeadersWrapper{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.headers
    }
}