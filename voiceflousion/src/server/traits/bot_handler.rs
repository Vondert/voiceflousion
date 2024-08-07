use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use crate::core::traits::Client;
use crate::errors::VoiceflousionResult;

/// Trait for functions that handles bot client interaction.
///
/// `BotHandler` is a trait that defines a handler for processing bot client interaction.
/// The handler is an asynchronous function that takes a client update and a reference to the client,
/// and returns a future that resolves to a `VoiceflousionResult`.
pub trait BotHandler<C: Client + 'static>: Fn(C::ClientUpdate<'static>, Arc<C>) -> Pin<Box<dyn Future<Output = VoiceflousionResult<()>> + Send>> + Send + Sync {}

/// Implementation of `BotHandler` for any function that matches the required signature.
///
/// This implementation allows any function or closure that matches the required signature to be used as a `BotHandler`.
impl<C, F> BotHandler<C> for F
where
    F: Fn(C::ClientUpdate<'static>, Arc<C>) -> Pin<Box<dyn Future<Output = VoiceflousionResult<()>> + Send>> + Send + Sync,
    C: Client + 'static,
{}