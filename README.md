# Voiceflousion

Voiceflousion is a framework designed to integrate chatbots from the Voiceflow chatbot constructor into any chat platform. Currently, it supports Telegram, with future plans for Instagram, WhatsApp, and more. The framework also provides tools for creating custom integrations for any chat platform, supporting message formats such as text, buttons, images, cards, and carousels.

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

### Steps

1. Clone the repository:
    ```sh
    git clone https://github.com/Vondert/voiceflousion.git
    cd voiceflousion
    ```

2. Navigate to the example directory:
    ```sh
    cd example
    ```

3. Rename `.env.example` to `.env` and fill in the required information:
    ```sh
    mv .env.example .env
    ```

4. Install dependencies and run the example:
    ```sh
    cargo run --release
    ```

For detailed setup and running instructions, refer to the [example/src/main.rs](example/src/main.rs) file.

## Dependencies

The dependencies required to run the project are listed in the [example/Cargo.toml](example/Cargo.toml) file. For custom integrations, you may need the `async-trait` crate.

## Usage

The example provided in the `example` directory demonstrates how to set up and run a Telegram bot integration. You can refer to this example as a template for creating your own integrations.

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