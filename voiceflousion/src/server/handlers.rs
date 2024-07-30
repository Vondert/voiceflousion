use std::future::Future;
use std::pin::Pin;
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

/// Base dialog handler that processes client updates.
///
/// This function is a default implementation for handling client updates. It prints the received update,
/// interacts with the client, and handles the response.
///
/// # Parameters
///
/// * `update` - The client update to process.
/// * `client` - The client to interact with.
///
/// # Returns
///
/// A `VoiceflousionResult` indicating the success or failure of the operation.
pub async fn base_dialog_handler<C: Client>(update: C::ClientUpdate<'_>, client: Arc<C>) -> VoiceflousionResult<()> {
    println!("Update: {:?}", &update);
    match client.interact_with_client(update, None).await {
        Ok(message) => {
            println!("Task: {:?}", message);
            Ok(())
        },
        Err(e) => {
            println!("Dialog: Error {:?}", e);
            Err(e)
        },
    }
}

