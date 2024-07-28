use std::sync::Arc;
use crate::core::traits::Client;

pub async fn base_dialog_handler<C: Client>(update: C::ClientUpdate<'_>, client: Arc<C>) -> () {
    println!("Update: {:?}", &update);
    match client.interact_with_client(update, None).await {
        Ok(message) => println!("Task: {:?}", message),
        Err(e) => {
            println!("Dialog: Error {:?}", e);
        },
    };
}