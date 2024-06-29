use std::env;
use dotenv::dotenv;
use crate::voiceflow::VoiceflowClient;
use crate::voiceflow::request_structures::Session;

mod voiceflow;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_id: String = env::var("BOT_ID").unwrap_or_else(|_| "".to_string());
    let version_id: String = env::var("VERSION_ID").unwrap_or_else(|_| "".to_string());
    let vf_api_key: String = env::var("VF_API_KEY").unwrap_or_else(|_| "".to_string());

    let voiceflow_client = VoiceflowClient::new(vf_api_key, bot_id, version_id);
    let session = Session::new("12321432434".to_string(), "123124324413".to_string());

    let _r = voiceflow_client.launch_dialog(&session, None).await;
    println!("Button resp");
    //voiceflow_client.send_message(&session, None, String::from("Buy")).await;
    //voiceflow_client.choose_button(&session, None, String::from("gds")).await;
}