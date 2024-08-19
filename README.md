# Voiceflousion

Voiceflousion is a framework designed to integrate chatbots from the Voiceflow chatbot constructor into any chat platform. Currently, it supports Voiceflow bots integration with Telegram and WhatsApp, with future plans for Instagram, Discord, and more. The framework also provides its own web server for launching chatbots and tools for creating custom integrations for any chat platform, supporting message formats such as text, buttons, images, cards, and carousels.

## Features

- **Telegram and WhatsApp Integration**: Supports text, buttons, images, cards, and carousels.
- **Custom Integrations**: Tools provided for developing integrations for any chat platform with minimal code.
- **Session Management**: Fully automated user and bot session management (creation, cleanup, validation, synchronization guarantees).
- **Scalability**: Supports multiple client bots for a single Voiceflow bot, and multiple Voiceflow bots and clients on a single server.
- **Admin Capabilities**: Features for creating an admin interface, such as retrieving all bot sessions, activating, and deactivating user sessions.
- **Extensibility**: Easily extendable to support additional platforms like Instagram and WhatsApp.
- **Multi-User Support**: The client bot supports multiple users simultaneously and offers flexible settings for the maximum number of users, session validity time, and cleanup interval.
- **Voiceflousion server**: Web server for launching and managing chatbots without needing of external dependencies.
- **Custom handlers**: Developer can write a custom function for processing bot's workflow, for example save conversation parts into database.
- **Server security opportunities**: Bot authentication tokens and allowed origins setting for Voiceflousion server.

## Installation and Setup

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)
- A ready Voiceflow bot, including its API key, bot ID, and version ID
- Telegram or WhatsApp bots created with their respective tools, including their API keys

### Environment Variables

It is recommended to store sensitive information like API keys in a `.env` file. Here is an example of the `.env` file content:

```plaintext
VF_API_KEY=VF.DM.xxxxxxxxxxxx.xxxxxxxxxx
BOT_ID=32487267c832hx4h248
VERSION_ID=27hd634532742g424234
TELEGRAM_BOT_TOKEN=4324234324:xxxxxxxxxxxxxxxx
WHATSAPP_BOT_TOKEN=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
WHATSAPP_BOT_ID=xxxxxxxxxxxxxxxxxxxx
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

## Adding Voiceflousion into Your Project (Telegram and WhatsApp)

### Set Up the Environment

Create a `.env` file in the root directory of your project with the necessary variables:

```
VF_API_KEY=VF.DM.xxxxxxxxxxxx.xxxxxxxxxx
BOT_ID=32487267c832hx4h248
VERSION_ID=27hd634532742g424234
TELEGRAM_BOT_TOKEN=4324234324:xxxxxxxxxxxxxxxx
WHATSAPP_BOT_TOKEN=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
WHATSAPP_BOT_ID=xxxxxxxxxxxxxxxxxxxx
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
tokio = { version = "1.36.0", features = ["rt", "rt-multi-thread", "macros"] }
voiceflousion = { version = "0.3.0", features = ["all-integrations", "server"] }
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
let whatsapp_bot_token = env::var("WHATSAPP_BOT_TOKEN").unwrap_or_else(|_| "".to_string());
let whatsapp_bot_id = env::var("WHATSAPP_BOT_ID").unwrap_or_else(|_| "".to_string());
```

**Initialize Voiceflow Client:** Create a new `VoiceflowClient` instance.

```rust
use std::sync::Arc;
use voiceflousion::core::voiceflow::VoiceflowClient;

let voiceflow_client = Arc::new(VoiceflowClient::new(vf_api_key, bot_id.clone(), version_id, 10, None));
```

**Build Telegram Client and WhatsApp Client:** Set up the `TelegramClient` and `WhatsAppClient` with session management.

```rust
use voiceflousion::core::ClientBuilder;
use voiceflousion::integrations::telegram::TelegramClient;

let client_builder = ClientBuilder::new(telegram_bot_id.clone(), telegram_bot_token.clone(), voiceflow_client.clone(), 10)
    .add_session_duration(120)
    .allow_sessions_cleaning(60);
let telegram_client = Arc::new(TelegramClient::new(client_builder));

let client_builder = ClientBuilder::new(whatsapp_bot_id.clone(), whatsapp_bot_token.clone(), voiceflow_client.clone(), 10)
.set_session_duration(120)
.allow_sessions_cleaning(60);
let whatsapp_client = WhatsAppClient::new(client_builder);
```

**Build Telegram Client and WhatsApp Client managers:** Set up the `ClientManager` with created clients

```rust
use voiceflousion::core::base_structs::ClientsManager;

let telegram_client_manager = Arc::new(ClientsManager::from_clients(vec![telegram_client]));
let whatsapp_client_manager = Arc::new(ClientsManager::from_clients(vec![whatsapp_client]));
```

**Set Up Voiceflousion server:**  Create `VoiceflousionServer` with previously built `ClientManager` instances and set up the base dialog handler for handling updates from clients.

```rust
use voiceflousion::server::handlers::base_dialog_handler;
use voiceflousion::server::VoiceflousionServer;

let telegram_voiceflousion_server = VoiceflousionServer::<TelegramClient>::new("telegram".to_string(), {
        |update, client| Box::pin(base_dialog_handler(update, client))
    })
    .set_clients_manager(telegram_client_manager);

let whatsapp_voiceflousion_server = VoiceflousionServer::<WhatsAppClient>::new("whatsapp".to_string(), {
        |update, client| Box::pin(base_dialog_handler(update, client))
    })
    .set_clients_manager(whatsapp_client_manager);
```

**Run the Server:** Start the server to listen for incoming webhook requests.

```rust
let telegram_voiceflousion_server_closure = async {
    telegram_voiceflousion_server.run(([127, 0, 0, 1], 8080)).await;
};

let whatsapp_voiceflousion_server_closure = async {
    whatsapp_voiceflousion_server.run(([127, 0, 0, 1], 8081)).await;
};

tokio::join!(telegram_voiceflousion_server_closure, whatsapp_voiceflousion_server_closure);
```

**Receive webhook address:** Copy given urls from console and set webhook with Telegram API and in WhatsApp application on Meta developers platform.

```plaintext
Server is set on 127.0.0.1:8080/telegram
Server is set on 127.0.0.1:8081/whatsapp
Bots without authentication token are available on 127.0.0.1:8080/telegram/<bot_id>
Bots with authentication token are available on 127.0.0.1:8080/telegram/<bot_id>/?token=<token>
Bots without authentication token are available on 127.0.0.1:8081/whatsapp/<bot_id>
Bots with authentication token are available on 127.0.0.1:8081/whatsapp/<bot_id>/?token=<token>
```

## Dependencies

The dependencies required to run the project are listed in the [example/Cargo.toml](example/Cargo.toml) file. For custom integrations, you may need the `async-trait` crate.

## Usage

By using this framework, you can easily and flexibly integrate your bots with pre-built integrations for Telegram and WhatsApp, create powerful and configurable bot management system. Voiceflousion also provides toolkit for creating custom integrations by implementing the `Client`, `Update`, `Sender`, and `Responder` traits. Once implemented, you can use your custom client in the same way as demonstrated in the example. For using `VoiceflousionServer` with your custom `Client` type you also need to implement `ServerClient` trait.

## Documentation

Crates.io: https://crates.io/crates/voiceflousion

Docs: https://docs.rs/voiceflousion/latest/voiceflousion/

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Vondert (Ivan Milennyi)

Github: https://github.com/Vondert

Linkedin: https://www.linkedin.com/in/ivan-milennyi-1084842a5/

X: https://x.com/Vondert1

Email: 25042018avic@gmail.com