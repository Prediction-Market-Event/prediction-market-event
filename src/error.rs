use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("serde_json failed to serialize/deserialize: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("nostr event: {0}")]
    NostrEvent(#[from] nostr::event::Error),

    #[error("nostr unsigned event: {0}")]
    NostrUnsignedEvent(#[from] nostr::event::unsigned::Error),

    #[error("nostr event builder: {0}")]
    NostrEventBuilder(#[from] nostr::event::builder::Error),

    #[error("nostr keys: {0}")]
    NostrKey(#[from] nostr::key::Error),
}
