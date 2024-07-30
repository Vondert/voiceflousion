use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::core::traits::Client;
use crate::errors::VoiceflousionResult;

pub trait BotHandler<C: Client + 'static>: Fn(C::ClientUpdate<'static>, Arc<C>) -> Pin<Box<dyn Future<Output = VoiceflousionResult<()>> + Send>> + Send + Sync {

}
impl<C, F> BotHandler<C> for F
where
    F: Fn(C::ClientUpdate<'static>, Arc<C>) -> Pin<Box<dyn Future<Output = VoiceflousionResult<()>> + Send>> + Send + Sync,
    C: Client + 'static ,
{}

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

pub fn wrap_handler<C: Client + 'static>(handler: impl BotHandler<C> + 'static) -> Arc<impl BotHandler<C>> {
    Arc::new(move |update: C::ClientUpdate<'static>, client: Arc<C>| {
        handler(update, client)
    })
}