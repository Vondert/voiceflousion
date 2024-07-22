use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use serde_json::Value;
use warp::Filter;
use voiceflousion::core::ClientBuilder;
use voiceflousion::core::traits::{Client, Update};
use voiceflousion::core::voiceflow::VoiceflowClient;
use voiceflousion::integrations::telegram::{TelegramClient, TelegramUpdate};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_id: String = env::var("BOT_ID").unwrap_or_else(|_| "".to_string());
    let version_id: String = env::var("VERSION_ID").unwrap_or_else(|_| "".to_string());
    let vf_api_key: String = env::var("VF_API_KEY").unwrap_or_else(|_| "".to_string());
    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| "".to_string());
    let telegram_bot_id = telegram_bot_token.split(':').next().unwrap().to_string();
    let voiceflow_client = Arc::new(VoiceflowClient::new(vf_api_key, bot_id.clone(), version_id, 10, None));

    let client_builder = ClientBuilder::new(telegram_bot_id.clone(), telegram_bot_token.clone(), voiceflow_client.clone(), 10)
        .add_session_duration(120)
        .allow_sessions_cleaning(60);
    let telegram_client = Arc::new(TelegramClient::new(client_builder));

    let webhook = warp::post()
        .and(warp::path("bot"))
        .and(warp::body::json())
        .and(warp::any().map(move || telegram_client.clone()))
        .and_then(handle_webhook);

    warp::serve(webhook)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
async fn handle_webhook(body: Value, client: Arc<TelegramClient>) -> Result<impl warp::Reply, warp::Rejection> {
    let update = match TelegramUpdate::from_request_body(body.clone()){
        Ok(update) => update,
        Err(err) => {
            println!("Error: {:?}", &err);
            return     Ok(warp::reply::json(&"Ok".to_string()))
        }
    };
    println!("Telegram update: {:?}", &update);

    match client.interact_with_client(update, None).await {
         Ok(message) => println!("Task: {:?}", message),
         Err(e) => {
             println!("Dialog: Error {:?}", e);
         },
     };



    Ok(warp::reply::json(&"Ok".to_string()))
}