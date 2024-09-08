use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("serde_json failed to serialize/deserialize: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("nostr event: {0}")]
    NostrEvent(#[from] nostr_sdk::nostr::event::Error),

    #[error("nostr event builder: {0}")]
    NostrEventBuilder(#[from] nostr_sdk::nostr::event::builder::Error),

    #[error("nostr keys: {0}")]
    NostrKey(#[from] nostr_sdk::nostr::key::Error),
}