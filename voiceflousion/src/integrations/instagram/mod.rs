mod instagram_client;
mod instagram_sender;
mod instagram_update;
mod instagram_responder;

pub use self::instagram_client::InstagramClient;
pub use self::instagram_update::InstagramUpdate;
pub use self::instagram_responder::InstagramResponder;

#[cfg(not(feature = "advanced"))]
mod utils;

#[cfg(feature = "advanced")]
pub mod utils;

#[cfg(feature = "advanced")]
pub use self::{
    instagram_sender::InstagramSender
};

#[cfg(not(feature = "advanced"))]
use self::{
    instagram_sender::InstagramSender
};