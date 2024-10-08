use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use tokio::join;
use voiceflousion::core::base_structs::ClientsManager;
use voiceflousion::core::ClientBuilder;
use voiceflousion::core::voiceflow::VoiceflowClient;
use voiceflousion::integrations::telegram::TelegramClient;
use voiceflousion::integrations::whatsapp::WhatsAppClient;
use voiceflousion::server::handlers::base_dialog_handler;
use voiceflousion::server::VoiceflousionServer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_id: String = env::var("BOT_ID").unwrap_or_else(|_| "".to_string());
    let version_id: String = env::var("VERSION_ID").unwrap_or_else(|_| "".to_string());
    let vf_api_key: String = env::var("VF_API_KEY").unwrap_or_else(|_| "".to_string());
    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| "".to_string());
    let telegram_bot_id = telegram_bot_token.split(':').next().unwrap().to_string();
    let whatsapp_bot_token = env::var("WHATSAPP_BOT_TOKEN").unwrap_or_else(|_| "".to_string());
    let whatsapp_bot_id = env::var("WHATSAPP_BOT_ID").unwrap_or_else(|_| "".to_string());

    let voiceflow_client = Arc::new(VoiceflowClient::new(vf_api_key, bot_id.clone(), version_id, 10, None));

    let client_builder = ClientBuilder::new(telegram_bot_id.clone(), telegram_bot_token.clone(), voiceflow_client.clone(), 10)
        .set_session_duration(120)
        .allow_sessions_cleaning(60);
    let telegram_client = TelegramClient::new(client_builder);
    let telegram_client_manager = Arc::new(ClientsManager::from_clients(vec![telegram_client]));

    let telegram_voiceflousion_server = VoiceflousionServer::<TelegramClient>::new({
        |update, client| Box::pin(base_dialog_handler(update, client))
    }).set_clients_manager(telegram_client_manager);

    let client_builder = ClientBuilder::new(whatsapp_bot_id.clone(), whatsapp_bot_token.clone(), voiceflow_client.clone(), 10)
        .set_session_duration(120)
        .allow_sessions_cleaning(60);
    let whatsapp_client = WhatsAppClient::new(client_builder);
    let whatsapp_client_manager = Arc::new(ClientsManager::from_clients(vec![whatsapp_client]));

    let whatsapp_voiceflousion_server = VoiceflousionServer::<WhatsAppClient>::new({
        |update, client| Box::pin(base_dialog_handler(update, client))
    }).set_clients_manager(whatsapp_client_manager);

    let telegram_voiceflousion_server_closure = async {
        telegram_voiceflousion_server
            .run(([127, 0, 0, 1], 8080))
            .await;
    };

    let whatsapp_voiceflousion_server_closure = async {
        whatsapp_voiceflousion_server
            .run(([127, 0, 0, 1], 8081))
            .await;
    };

    join!(telegram_voiceflousion_server_closure, whatsapp_voiceflousion_server_closure);
}