#[cfg(feature = "telegram")]
pub mod telegram;
#[cfg(feature = "whatsapp")]
pub mod whatsapp;

#[cfg(not(feature = "advanced"))]
pub(crate) mod utils;

#[cfg(feature = "advanced")]
pub mod utils;

pub mod discord;