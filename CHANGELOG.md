# Changelog

## [0.2.0] - 2024-08-04
### Added
- Added `ClientManager` struct for managing large amounts of clients.
- Created `server` module.
- Created `VoiceflowServer` (web server for bot webhooks) for setting up bots.
- Added status and methods for working with it to `ClientBase` struct.
- Added `Debug` trait bound for `Responder` and `Update` traits.
- Added `handlers` module to `server` module.
- Added `BotHandler` trait for handlers.
- Created `BotAuthToken` struct for secret token.
- Added `bot_auth_token` field to `ClientBase` and `ClientBuilder`.
- Added methods for controlling the session cleanup process to `SessionManager`.
- Added `VoiceflowResponseBlockDeserializer` struct.
- Added `ORIGINS` constant to `Client` trait.
- Added `endpoints` module to `server` module.
- Added `payload` field to `VoiceflowButton`.
- Added buttons payload support for Telegram integration and `Client` trait methods.
- Added 'advanced' feature for developers.
- Added 'server' feature for using `VoiceflousionServer`.

### Changed
- Refactored and improved `build_message()` method in `VoiceflowMessageBuilder` struct.
- Improved `VoiceflowSession::from_chat_id()` method to ensure there are no collision scenarios.
- Improved `VoiceflowClient`'s error responses.
- Improved `VoiceflowButton` deserialization.

### Fixed
- Fixed `SessionManager` creation.
- Fixed critical issue in `is_valid_session()` method from `SessionMap`.

## [0.1.2] - 2024-07-26
### Added
- Added `VoiceflousionResult` type for `Result` aliasing.
- Added `base_structs` module to `core`.

### Changed
- `ClientBase` changed from trait to struct.
- Refactored and improved `Update` trait. Created `UpdateBase` struct.
- Refactored and improved `Sender` trait. Created `SenderBase` struct.
- Refactored and improved `Responder` trait. Created `ResponderBase` struct.

## [0.1.1] - 2024-07-24
### Added
- Added `launch_state` for clients.

## [0.1.0] - 2024-07-23
### Added
- Added `core` module.
- Added `voiceflow` module to `core` for handling Voiceflow integration.
- Added `traits` module to `core` to provide structure and methods for future integrations.
- Added `session_wrappers` module to `core` to provide session handling and storing logic.
- Added `integrations` module to `core` for future integrations.
- Added `telegram` module to `integrations` to provide Telegram integration.
- Added `errors` module to `core` for error types.
