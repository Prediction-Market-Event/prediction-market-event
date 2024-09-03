use nostr_sdk::{JsonUtil, Keys, Kind, Tag, TagStandard};

use crate::{Event, EventPayout};

pub struct NewEvent;

impl NewEvent {
    pub const NOSTR_KIND: Kind = Kind::Custom(6275);

    pub fn create_nostr_event_json(event: &Event, secret_key: &str) -> Result<String, String> {
        let event_json = event.try_to_json_string()?;

        let e_hash_hex = event
            .hash_sha256_hex()
            .map_err(|e| format!("failed to get sha256 hex of event: {e}"))?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(e_hash_hex).into()];

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

pub struct FutureEventPayoutAttestationPledge;

impl FutureEventPayoutAttestationPledge {
    pub const NOSTR_KIND: Kind = Kind::Custom(6276);

    pub fn create_nostr_event_json(event: &Event, secret_key: &str) -> Result<String, String> {
        let e_hash_hex = event
            .hash_sha256_hex()
            .map_err(|e| format!("failed to get sha256 hex of event: {e}"))?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(e_hash_hex.clone()).into()];

        let builder = nostr_sdk::EventBuilder::new(Self::NOSTR_KIND, e_hash_hex, tags);

        let keys = Keys::parse(secret_key)
            .map_err(|e| format!("failed to parse nostr secret key: {e}"))?;
        let event = builder
            .to_event(&keys)
            .map_err(|e| format!("failed to create nostr event: {e}"))?;

        event
            .try_as_json()
            .map_err(|e| format!("failed nostr event conversion to json: {e}"))
    }

    /// Returns serialized nostr public key and the hex hash of the [Event] it pledges to make a payout attestation to.
    /// IMPORTANT: EventPayout is not validated.
    pub fn interpret_nostr_event_json(json: &str) -> Result<([u8; 32], String), String> {
        let nostr_event = nostr_sdk::Event::from_json(json)
            .map_err(|e| format!("failed to parse nostr event from json: {e}"))?;
        _ = nostr_event
            .verify()
            .map_err(|e| format!("failed to verify nostr event: {e}"))?;

        let nostr_public_key = nostr_event.pubkey.serialize();
        let event_hex_hash = nostr_event.content().to_string();
        if event_hex_hash.len() != 64
            || matches!(
                event_hex_hash.find(|c: char| !c.is_ascii_hexdigit()),
                Some(_)
            )
        {
            return Err(format!(
                "nostr event content does not have format of event hash hex"
            ));
        }

        Ok((nostr_public_key, event_hex_hash))
    }
}

pub struct EventPayoutAttestation;

impl EventPayoutAttestation {
    pub const NOSTR_KIND: Kind = Kind::Custom(6277);

    pub fn create_nostr_event_json(
        event_payout: &EventPayout,
        secret_key: &str,
    ) -> Result<String, String> {
        let event_payout_json = event_payout.try_to_json_string()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_payout.event_hash_hex.clone()).into()];

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
    pub fn interpret_nostr_event_json(json: &str) -> Result<([u8; 32], EventPayout), String> {
        let nostr_event = nostr_sdk::Event::from_json(json)
            .map_err(|e| format!("failed to parse nostr event from json: {e}"))?;
        _ = nostr_event
            .verify()
            .map_err(|e| format!("failed to verify nostr event: {e}"))?;

        let nostr_public_key = nostr_event.pubkey.serialize();
        let event_payout = EventPayout::try_from_json_str(nostr_event.content())
            .map_err(|e| format!("failed to parse event payout from nostr event content: {e}"))?;

        Ok((nostr_public_key, event_payout))
    }
}
