use std::env;
use dotenv::dotenv;
use crate::integrations::{TelegramClient, TelegramSession};
use crate::voiceflow::VoiceflowClient;
use crate::voiceflow::request_structures::VoiceflowSession;

mod voiceflow;
mod integrations;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_id: String = env::var("BOT_ID").unwrap_or_else(|_| "".to_string());
    let version_id: String = env::var("VERSION_ID").unwrap_or_else(|_| "".to_string());
    let vf_api_key: String = env::var("VF_API_KEY").unwrap_or_else(|_| "".to_string());
    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| "".to_string());
    let voiceflow_client = VoiceflowClient::new(vf_api_key, bot_id, version_id);
    let telegram_client = TelegramClient::new(telegram_bot_token, &voiceflow_client);
    let telegram_session = TelegramSession::from_chat_id("asdjdsfksdfkdsjf".to_string());

    let _r = telegram_client.launch_voiceflow_dialog(&telegram_session, None).await;
    println!("Button resp");
    //voiceflow_client.send_message(&session, None, String::from("Buy")).await;
    //voiceflow_client.choose_button(&session, None, String::from("gds")).await;
}