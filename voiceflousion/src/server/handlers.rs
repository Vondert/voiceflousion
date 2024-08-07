use std::sync::Arc;
use crate::core::traits::Client;
use crate::errors::VoiceflousionResult;

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

