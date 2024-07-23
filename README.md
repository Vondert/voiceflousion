# Voiceflousion

Voiceflousion is a framework designed to integrate chatbots from the Voiceflow chatbot constructor into any chat platform. Currently, it supports Voiceflow bots integration with Telegram, with future plans for Instagram, WhatsApp, and more. The framework also provides tools for creating custom integrations for any chat platform, supporting message formats such as text, buttons, images, cards, and carousels.

## Features

- **Telegram Integration**: Supports text, buttons, images, cards, and carousels.
- **Custom Integrations**: Tools provided for developing integrations for any chat platform with minimal code.
- **Session Management**: Fully automated user and bot session management (creation, cleanup, validation, synchronization guarantees).
- **Scalability**: Supports multiple client bots for a single Voiceflow bot, and multiple Voiceflow bots and clients on a single server.
- **Admin Capabilities**: Features for creating an admin interface, such as retrieving all bot sessions, activating, and deactivating user sessions.
- **Extensibility**: Easily extendable to support additional platforms like Instagram and WhatsApp.
- **Multi-User Support**: The client bot supports multiple users simultaneously and offers flexible settings for the maximum number of users, session validity time, and cleanup interval.
- **Crates.io Release**: Future plans include releasing the framework on crates.io.

## Installation and Setup

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)
- A ready Voiceflow bot, including its API key, bot ID, and version ID
- A Telegram bot created with BotFather, including its API key
- A webhook URL for the Telegram bot, set up using the Telegram API

### Environment Variables

It is recommended to store sensitive information like API keys in a `.env` file. Here is an example of the `.env` file content:

```plaintext
VF_API_KEY=VF.DM.xxxxxxxxxxxx.xxxxxxxxxx
BOT_ID=32487267c832hx4h248
VERSION_ID=27hd634532742g424234
TELEGRAM_BOT_TOKEN=4324234324:xxxxxxxxxxxxxxxx
```

## Quick Start: Running the Example Project

### Clone the repository:

```sh
git clone https://github.com/Vondert/voiceflousion.git
cd voiceflousion
```

### Navigate to the example directory:

```sh
cd example
```

### Rename `.env.example` to `.env` and fill in the required information:

```sh
mv .env.example .env
```

### Install dependencies and run the example:

```sh
cargo run --release
```

## Adding Voiceflousion into Your Project

### Set Up the Environment

Create a `.env` file in the root directory of your project with the necessary variables:

```
VF_API_KEY=VF.DM.xxxxxxxxxxxx.xxxxxxxxxx
BOT_ID=32487267c832hx4h248
VERSION_ID=27hd634532742g424234
TELEGRAM_BOT_TOKEN=4324234324:xxxxxxxxxxxxxxxx
```

### Add Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[package]
name = "example"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0.193", features = ["derive"] }
dotenv = "0.15.0"
serde_json = "1.0.114"
warp = "0.3.7"
tokio = { version = "1.36.0", features = ["rt", "rt-multi-thread", "macros"] }
voiceflousion = { path = "../voiceflousion", features = ["all-integrations"] }
```

### Set Up the Main File

**Load Environment Variables:** Use dotenv to load environment variables from the `.env` file.

```rust
use dotenv::dotenv;
dotenv().ok();
```

**Retrieve Environment Variables:** Get the necessary API keys and IDs from the environment.

```rust
use std::env;
let bot_id: String = env::var("BOT_ID").unwrap_or_else(|_| "".to_string());
let version_id: String = env::var("VERSION_ID").unwrap_or_else(|_| "".to_string());
let vf_api_key: String = env::var("VF_API_KEY").unwrap_or_else(|_| "".to_string());
let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| "".to_string());
let telegram_bot_id = telegram_bot_token.split(':').next().unwrap().to_string();
```

**Initialize Voiceflow Client:** Create a new `VoiceflowClient` instance.

```rust
use std::sync::Arc;
use voiceflousion::core::voiceflow::VoiceflowClient;
let voiceflow_client = Arc::new(VoiceflowClient::new(vf_api_key, bot_id.clone(), version_id, 10, None));
```

**Build Telegram Client:** Set up the `TelegramClient` with session management.

```rust
use voiceflousion::core::ClientBuilder;
use voiceflousion::integrations::telegram::TelegramClient;
let client_builder = ClientBuilder::new(telegram_bot_id.clone(), telegram_bot_token.clone(), voiceflow_client.clone(), 10)
    .add_session_duration(120)
    .allow_sessions_cleaning(60);
let telegram_client = Arc::new(TelegramClient::new(client_builder));
```

**Set Up Webhook:** Define the webhook endpoint using Warp (or any other web framework of your choice).

```rust
use warp::Filter;
let webhook = warp::post()
    .and(warp::path("bot"))
    .and(warp::body::json())
    .and(warp::any().map(move || telegram_client.clone()))
    .and_then(handle_webhook);
```

**Run the Server:** Start the server to listen for incoming webhook requests.

```rust
warp::serve(webhook)
    .run(([127, 0, 0, 1], 8080))
    .await;
```

**Handle Webhook Requests:** Define the `handle_webhook` function to process incoming Telegram updates.

```rust
use serde_json::Value;
use voiceflousion::core::traits::Update;
use voiceflousion::integrations::telegram::TelegramUpdate;

async fn handle_webhook(body: Value, client: Arc<TelegramClient>) -> Result<impl warp::Reply, warp::Rejection> {
    let update = match TelegramUpdate::from_request_body(body.clone()) {
        Ok(update) => update,
        Err(err) => {
            println!("Error: {:?}", &err);
            return Ok(warp::reply::json(&"Ok".to_string()));
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
```

## Dependencies

The dependencies required to run the project are listed in the [example/Cargo.toml](example/Cargo.toml) file. For custom integrations, you may need the `async-trait` crate.

## Usage

By using this framework, you can easily and flexibly integrate your bots with pre-built integrations for Telegram and Instagram. Voiceflousion also provides toolkit for creating custom integrations by implementing the `Client`, `Update`, `Sender`, and `Responder` traits. Once implemented, you can use your custom client in the same way as demonstrated in the example.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Vondert

Github: https://github.com/Vondert

Linkedin: https://www.linkedin.com/in/ivan-milennyi-1084842a5/

X: https://x.com/Vondert1

Email: 25042018avic@gmail.com