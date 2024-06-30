use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use tokio::task;
use crate::integrations::{Session, TelegramClient, TelegramSession};
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
    let voiceflow_client = Arc::new(VoiceflowClient::new(vf_api_key, bot_id, version_id));

    let chat_id = format!("gdfgfdgfdgfdg");
    let telegram_session = TelegramSession::from_chat_id(chat_id.clone());
    let mut telegram_sessions: Vec<TelegramSession> = Vec::new();
    telegram_sessions.push(telegram_session);
    let telegram_client = Arc::new(TelegramClient::new(telegram_bot_token, voiceflow_client.clone(), Some(telegram_sessions)));
    let result = telegram_client.launch_voiceflow_dialog(chat_id, None).await;
    match result {
        Ok(message) => println!("Task: {:?}", message),
        Err(e) => println!("Task: Error {:?}", e),
    }
    /*let mut handles = vec![];
    for i in 0..5 {
        let telegram_client = telegram_client.clone();
        let handle = task::spawn(async move {
            let chat_id = format!("gdfgfdgfdgfdg");
            let result = telegram_client.launch_voiceflow_dialog(chat_id, None).await;
            match result {
                Ok(message) => println!("Task {}: {:?}", i, message),
                Err(e) => println!("Task {}: Error {:?}", i, e),
            }
        });
        handles.push(handle);
    }

    // Ожидание завершения всех задач
    for handle in handles {
        if let Err(e) = handle.await {
            println!("Task failed: {:?}", e);
        }
    }*/
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

    println!("Button resp");
    //voiceflow_client.send_message(&session, None, String::from("Buy")).await;
    //voiceflow_client.choose_button(&session, None, String::from("gds")).await;
}