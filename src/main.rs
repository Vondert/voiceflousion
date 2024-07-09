use std::env;
use std::sync::Arc;
use chrono::Utc;
use dotenv::dotenv;
use tokio::task;
use crate::integrations::telegram::{TelegramClient, TelegramUpdate, TelegramSession};
use crate::integrations::utils::InteractionType;
use crate::integrations::utils::traits::{Client, Update};
use crate::voiceflow::VoiceflowClient;

mod voiceflow;
mod integrations;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_id: String = env::var("BOT_ID").unwrap_or_else(|_| "".to_string());
    let version_id: String = env::var("VERSION_ID").unwrap_or_else(|_| "".to_string());
    let vf_api_key: String = env::var("VF_API_KEY").unwrap_or_else(|_| "".to_string());
    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| "".to_string());
    let voiceflow_client = Arc::new(VoiceflowClient::new(vf_api_key, bot_id.clone(), version_id, 10));

    let chat_id = format!("510947895");
    let telegram_client = Arc::new(TelegramClient::new(telegram_bot_token, voiceflow_client.clone(), None, None, 10));

    let now = Utc::now().timestamp();
    let update =  TelegramUpdate::new(bot_id.clone(), chat_id.clone(), now, InteractionType::new(String::new(), None));
    let result = telegram_client.interact_with_client(update, None, None).await;
    match result {
        Ok(message) => println!("Task: {:?}", message),
        Err(e) => println!("Task: Error {:?}", e),
    }
    // let now = Utc::now().timestamp();
    // let update =  TelegramUpdate::new(bot_id.clone(), chat_id.clone(), now, InteractionType::new(String::from("buy"), None));
    // let result = telegram_client.interact_with_client(update, None, None).await;
    // match result {
    //     Ok(message) => println!("Task: {:?}", message),
    //     Err(e) => println!("Task: Error {:?}", e),
    // }
    // let now = Utc::now().timestamp();
    // let update =  TelegramUpdate::new(bot_id.clone(), chat_id.clone(), now, InteractionType::new(String::from("fdgdfgdfg"), Some(String::from("fdgdfgdfg-aopiakpe"))));
    // let result = telegram_client.interact_with_client(update, None, None).await;
    // match result {
    //     Ok(message) => println!("Task: {:?}", message),
    //     Err(e) => println!("Task: Error {:?}", e),
    // }

    // let now = Utc::now().timestamp();
    // let update =  TelegramUpdate::new(bot_id.clone(), chat_id.clone(), now, InteractionType::new(String::from("How can I buy?"), String::from("text")));
    // let result = telegram_client.interact_with_client(update, None, None).await;
    // match result {
    //     Ok(message) => println!("Task: {:?}", message),
    //     Err(e) => println!("Task: Error {:?}", e),
    // }
    // let now = Utc::now().timestamp();
    // let update =  TelegramUpdate::new(bot_id.clone(), chat_id.clone(), now, InteractionType::new(String::from("Who are you?"), String::from("text")));
    // let result = telegram_client.interact_with_client(update, None, None).await;
    // match result {
    //     Ok(message) => println!("Task: {:?}", message),
    //     Err(e) => println!("Task: Error {:?}", e),
    // }
    // let now = Utc::now().timestamp();
    // let update =  TelegramUpdate::new(bot_id.clone(), chat_id.clone(), now, InteractionType::new(String::from("I want buy"), String::from("text")));
    // let result = telegram_client.interact_with_client(update, None, None).await;
    // match result {
    //     Ok(message) => println!("Task: {:?}", message),
    //     Err(e) => println!("Task: Error {:?}", e),
    // }


    // let mut handles = vec![];
    // for i in 0..5 {
    //     let telegram_client = telegram_client.clone();
    //     let handle = task::spawn(async move {
    //         let chat_id = format!("rgfdgfdgfdg");
    //         let bot_id = format!("gdfgdfgfdg");
    //         let now = Utc::now().timestamp();
    //         let update =  TelegramUpdate::new(bot_id.clone(), chat_id.clone(), now, InteractionType::new(String::new(), String::from("text")));
    //         let result = telegram_client.interact_with_client(update, None, None).await;
    //         match result {
    //             Ok(message) => println!("Task {}: {:?}", i, message),
    //             Err(e) => println!("Task {}: Error {:?}", i, e),
    //         }
    //     });
    //     handles.push(handle);
    // }
    // for i in 0..5 {
    //     let telegram_client = telegram_client.clone();
    //     let handle = task::spawn(async move {
    //         let chat_id = format!("sfdsdfsdf");
    //         let bot_id = format!("gdfgdfgfdg");
    //         let now = Utc::now().timestamp();
    //         let update =  TelegramUpdate::new(bot_id.clone(), chat_id.clone(), now, InteractionType::new(String::new(), String::from("text")));
    //         let result = telegram_client.interact_with_client(update, None, None).await;
    //         match result {
    //             Ok(message) => println!("Task {}: {:?}", i, message),
    //             Err(e) => println!("Task {}: Error {:?}", i, e),
    //         }
    //     });
    //     handles.push(handle);
    // }
    // //Ожидание завершения всех задач
    // for handle in handles {
    //     if let Err(e) = handle.await {
    //         println!("Task failed: {:?}", e);
    //     }
    // }

    // let mut handles = vec![];
    // for i in 0..2 {
    //     let telegram_client = telegram_client.clone();
    //     let handle = task::spawn(async move {
    //         let chat_id = format!("rgfdgfdgfdg");
    //         let now = Utc::now().timestamp();
    //         let result = telegram_client.interact_with_client(chat_id, now, String::from("How can I buy?"), None, None).await;
    //         match result {
    //             Ok(message) => println!("Task {}: {:?}", i, message),
    //             Err(e) => println!("Task {}: Error {:?}", i, e),
    //         }
    //     });
    //     handles.push(handle);
    // }
    //
    // // Ожидание завершения всех задач
    // for handle in handles {
    //     if let Err(e) = handle.await {
    //         println!("Task failed: {:?}", e);
    //     }
    // }


    /*let chat_id = format!("gdfgfdgfdgfdg");
    let result = telegram_client.launch_voiceflow_dialog(chat_id, None).await;
    match result {
        Ok(message) => println!("Task {}: {:?}", 1, message),
        Err(e) => println!("Task {}: Error {:?}", 1, e),
    }
    let chat_id = format!("gdfgfdgfdgfdg");
    let result = telegram_client.launch_voiceflow_dialog(chat_id, None).await;
    match result {
        Ok(message) => println!("Task {}: {:?}", 2, message),
        Err(e) => println!("Task {}: Error {:?}", 2, e),
    }
    let chat_id = format!("gdfgfdgfdgfdg");
    let result = telegram_client.launch_voiceflow_dialog(chat_id, None).await;
    match result {
        Ok(message) => println!("Task {}: {:?}", 3, message),
        Err(e) => println!("Task {}: Error {:?}", 3, e),
    }
    let chat_id = format!("gdfgfdgfdgfdg");
    let result = telegram_client.launch_voiceflow_dialog(chat_id, None).await;
    match result {
        Ok(message) => println!("Task {}: {:?}", 4, message),
        Err(e) => println!("Task {}: Error {:?}", 4, e),
    }*/

    //voiceflow_client.send_message(&session, None, String::from("Buy")).await;
    //voiceflow_client.choose_button(&session, None, String::from("gds")).await;
}