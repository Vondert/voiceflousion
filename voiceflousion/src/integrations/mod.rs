#[cfg(feature = "telegram")]
pub mod telegram;
#[cfg(feature = "whatsapp")]
pub mod whatsapp;
#[cfg(feature = "discord_unimplemented")]
pub mod discord;

#[cfg(not(feature = "advanced"))]
pub(crate) mod utils;

#[cfg(feature = "advanced")]
pub mod utils;