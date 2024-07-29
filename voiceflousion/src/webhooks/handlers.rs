use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::core::traits::Client;
use crate::errors::VoiceflousionResult;

pub type BotHandler<U, C> = dyn Fn(U, Arc<C>) -> Pin<Box<dyn Future<Output=VoiceflousionResult<()>> + Send>> + Send + Sync;

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

pub fn wrap_handler<C: Client + 'static>(handler: impl Fn(C::ClientUpdate<'static>, Arc<C>) -> Pin<Box<dyn Future<Output = VoiceflousionResult<()>> + Send>> + Clone + Send + Sync + 'static) -> Arc<BotHandler<C::ClientUpdate<'static>, C>> {
    Arc::new(move |update: C::ClientUpdate<'static>, client: Arc<C>| {
        let handler = handler.clone();
        Box::pin(async move {
            handler(update, client).await
        })
    })
}