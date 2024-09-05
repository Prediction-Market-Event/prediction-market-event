use std::fmt::Display;

use nostr_sdk::{JsonUtil, Keys, Kind, Tag, TagStandard};
use serde::{Deserialize, Serialize};

use crate::{Event, EventHashHex, EventPayout};

/// Nostr event containing a prediction market [Event]
pub struct NewEvent;

impl NewEvent {
    pub const NOSTR_KIND: Kind = Kind::Custom(6275);

    /// Creates [NewEvent] nostr event json
    pub fn create_nostr_event_json(event: &Event, secret_key: &str) -> Result<String, String> {
        let event_json = event.try_to_json_string()?;

        let e_hash_hex = event
            .hash_hex()
            .map_err(|e| format!("failed to get sha256 hex of event: {e}"))?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(e_hash_hex.0).into()];

        let builder = nostr_sdk::EventBuilder::new(Self::NOSTR_KIND, event_json, tags);

        let keys = Keys::parse(secret_key)
            .map_err(|e| format!("failed to parse nostr secret key: {e}"))?;
        let event = builder
            .to_event(&keys)
            .map_err(|e| format!("failed to create nostr event: {e}"))?;

        event
            .try_as_json()
            .map_err(|e| format!("failed nostr event conversion to json: {e}"))
    }

    /// Returns the [Event] found in nostr event.
    /// IMPORTANT: the returned [Event] is not validated.
    pub fn interpret_nostr_event_json(json: &str) -> Result<Event, String> {
        let nostr_event = nostr_sdk::Event::from_json(json)
            .map_err(|e| format!("failed to parse nostr event from json: {e}"))?;
        _ = nostr_event
            .verify()
            .map_err(|e| format!("failed to verify nostr event: {e}"))?;

        Event::try_from_json_str(nostr_event.content())
            .map_err(|e| format!("failed to parse event from nostr event content: {e}"))
    }
}

/// Nostr event that pledges the event signer will make an [EventPayout] for a specific [Event] in the future.
pub struct FutureEventPayoutAttestationPledge;

impl FutureEventPayoutAttestationPledge {
    pub const NOSTR_KIND: Kind = Kind::Custom(6276);

    /// Creates [FutureEventPayoutAttestationPledge] nostr event json
    pub fn create_nostr_event_json(event: &Event, secret_key: &str) -> Result<String, String> {
        let e_hash_hex = event
            .hash_hex()
            .map_err(|e| format!("failed to get sha256 hex of event: {e}"))?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(e_hash_hex.0.clone()).into()];

        let builder = nostr_sdk::EventBuilder::new(Self::NOSTR_KIND, e_hash_hex.0, tags);

        let keys = Keys::parse(secret_key)
            .map_err(|e| format!("failed to parse nostr secret key: {e}"))?;
        let event = builder
            .to_event(&keys)
            .map_err(|e| format!("failed to create nostr event: {e}"))?;

        event
            .try_as_json()
            .map_err(|e| format!("failed nostr event conversion to json: {e}"))
    }

    /// Returns nostr public key hex and the hex hash of the [Event] it pledges to make a payout attestation to.
    pub fn interpret_nostr_event_json(json: &str) -> Result<(NostrPublicKeyHex, EventHashHex), String> {
        let nostr_event = nostr_sdk::Event::from_json(json)
            .map_err(|e| format!("failed to parse nostr event from json: {e}"))?;
        _ = nostr_event
            .verify()
            .map_err(|e| format!("failed to verify nostr event: {e}"))?;

        let nostr_public_key_hex = nostr_event.pubkey.to_hex();
        let event_hash_hex = nostr_event.content().to_string();
        if !EventHashHex::is_hash_hex(&event_hash_hex) {
            return Err(format!(
                "nostr event content does not have format of event hash hex"
            ));
        }

        Ok((NostrPublicKeyHex(nostr_public_key_hex), EventHashHex(event_hash_hex)))
    }
}

/// Nostr event that contains an [EventPayout] attestation
pub struct EventPayoutAttestation;

impl EventPayoutAttestation {
    pub const NOSTR_KIND: Kind = Kind::Custom(6277);

    /// Creates [EventPayoutAttestation] nostr event json
    pub fn create_nostr_event_json(
        event_payout: &EventPayout,
        secret_key: &str,
    ) -> Result<String, String> {
        let event_payout_json = event_payout.try_to_json_string()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_payout.event_hash_hex.0.clone()).into()];

        let builder = nostr_sdk::EventBuilder::new(Self::NOSTR_KIND, event_payout_json, tags);

        let keys = Keys::parse(secret_key)
            .map_err(|e| format!("failed to parse nostr secret key: {e}"))?;
        let event = builder
            .to_event(&keys)
            .map_err(|e| format!("failed to create nostr event: {e}"))?;

        event
            .try_as_json()
            .map_err(|e| format!("failed nostr event conversion to json: {e}"))
    }

    /// Returns serialized nostr public key and the [EventPayout] it signed.
    /// IMPORTANT: EventPayout is not validated.
    pub fn interpret_nostr_event_json(json: &str) -> Result<(NostrPublicKeyHex, EventPayout), String> {
        let nostr_event = nostr_sdk::Event::from_json(json)
            .map_err(|e| format!("failed to parse nostr event from json: {e}"))?;
        _ = nostr_event
            .verify()
            .map_err(|e| format!("failed to verify nostr event: {e}"))?;

        let nostr_public_key_hex = nostr_event.pubkey.to_hex();
        let event_payout = EventPayout::try_from_json_str(nostr_event.content())
            .map_err(|e| format!("failed to parse event payout from nostr event content: {e}"))?;

        Ok((NostrPublicKeyHex(nostr_public_key_hex), event_payout))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NostrPublicKeyHex(pub String);

impl Display for NostrPublicKeyHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}