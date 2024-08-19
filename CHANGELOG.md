# Changelog

## [0.3.0] - 2024-08-19
### Added
- `whatsapp` module to `integrations` to provide WhatsApp integration.
- `traits` module in `server`.
- `ServerClient` trait in `server/traits` module and implemented it for existing Clients.
- `get_auth_endpoint()` and `authenticate_request` to `endpoints` module.
- `GET` endpoint to `VoiceflousionServer`.
- `subtypes` module in `server.
- `AuthResult` enum and `QueryParams` struct in `server/subtypes`.
- Placeholder text for buttons in `VoiceflowButtons` and `VoiceflowCards`.
- Timestamp mark into `VoiceflowCarousel`.
- `ValidationError` to `VoiceflousionError` enum.
- Serializers struct for integrations to unload `Sender` methods bodies.
- `get_button()` method to `SentMessage` struct.
- `CarouselSwitch` variant to `InteractionType` enum.
- Support for `VoiceflousionCarousel` without images.
- `chat_id` field and getter method `chat_id()` to `ResponderBase`.
- `whatsapp` compilation feature.
- `len()`, `push()`, `shift_block()` to `VoiceflowMessage`.
- Methods and field for managing and selecting cards in `VoiceflowCarousel`.
 
### Changed
- Refactored `main_endpoint()` function.
- `C` type from `Client` to `ServerClient` in `VoiceflousionServer`.
- Fully reworked buttons interaction logic.
- Carousel handling implementation.
- `Client` trait generic implementations of `choose_button_in_voiceflow_dialog()` and `interact_with_client()` methods.
- Moved `ORIGINS` constant from `Client` to `ServerClient`.
- Refactored `TelegramSender`.
- Banned `VoiceflowCards` without text. Text will be replaced with placeholder.
- Type of `allowed_origin_headers` field in `VoiceflousionServer` from `Vec` to `HashMap<&str, ()>`.
- Encapsulated `VoiceflowButtonActionType` enum inside crate.

### Removed
- `Image` variant from `VoiceflowButtonsOption` enum.
- `Deref` and `DerefMut` from `VoiceflowMessage`.

## [0.2.0] - 2024-08-04
### Added
- `ClientManager` struct for managing large amounts of clients.
- `server` module.
- `VoiceflowServer` (web server for bot webhooks) for setting up bots.
- Status and methods for working with it to `ClientBase` struct.
- `Debug` trait bound for `Responder` and `Update` traits.
- `handlers` module to `server` module.
- `BotHandler` trait for handlers.
- `BotAuthToken` struct for secret token.
- `bot_auth_token` field to `ClientBase` and `ClientBuilder`.
- Methods for controlling the session cleanup process to `SessionManager`.
- `VoiceflowResponseBlockDeserializer` struct.
- `ORIGINS` constant to `Client` trait.
- `endpoints` module to `server` module.
- `payload` field to `VoiceflowButton`.
- Buttons payload support for Telegram integration and `Client` trait methods.
- 'advanced' feature for developers.
- 'server' feature for using `VoiceflousionServer`.

### Changed
- Refactored and improved `build_message()` method in `VoiceflowMessageBuilder` struct.
- Improved `VoiceflowSession::from_chat_id()` method to ensure there are no collision scenarios.
- Improved `VoiceflowClient`'s error responses.
- Improved `VoiceflowButton` deserialization.

### Fixed
- `SessionManager` creation.
- Critical issue in `is_valid_session()` method from `SessionMap`.

## [0.1.2] - 2024-07-26
### Added
- `VoiceflousionResult` type for `Result` aliasing.
- `base_structs` module to `core`.

### Changed
- `ClientBase` changed from trait to struct.
- Refactored and improved `Update` trait. Created `UpdateBase` struct.
- Refactored and improved `Sender` trait. Created `SenderBase` struct.
- Refactored and improved `Responder` trait. Created `ResponderBase` struct.

## [0.1.1] - 2024-07-24
### Added
- `launch_state` for clients.

## [0.1.0] - 2024-07-23
### Added
- `core` module.
- `voiceflow` module to `core` for handling Voiceflow integration.
- `traits` module to `core` to provide structure and methods for future integrations.
- `session_wrappers` module to `core` to provide session handling and storing logic.
- `integrations` module to `core` for future integrations.
- `telegram` module to `integrations` to provide Telegram integration.
- `errors` module to `core` for error types.
