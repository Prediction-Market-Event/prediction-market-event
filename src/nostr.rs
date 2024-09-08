use std::fmt::Display;

use nostr_sdk::{JsonUtil, Keys, Kind, Tag, TagStandard};
use serde::{Deserialize, Serialize};

use crate::{Error, Event, EventHashHex, EventPayout};

/// Nostr event containing a prediction market [Event]
pub struct NewEvent;

impl NewEvent {
    pub const NOSTR_KIND: Kind = Kind::Custom(6275);

    /// Creates [NewEvent] nostr event json
    pub fn create_nostr_event_json(event: &Event, secret_key: &str) -> Result<String, Error> {
        let event_json = event.try_to_json_string()?;

        let e_hash_hex = event.hash_hex()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(e_hash_hex.0).into()];

        let builder = nostr_sdk::EventBuilder::new(Self::NOSTR_KIND, event_json, tags);

        let keys = Keys::parse(secret_key).map_err(|e| Error::from(e))?;
        let event = builder.to_event(&keys).map_err(|e| Error::from(e))?;

        event.try_as_json().map_err(|e| Error::from(e))
    }

    /// Returns the [Event] found in nostr event.
    /// IMPORTANT: the returned [Event] is not validated.
    pub fn interpret_nostr_event_json(json: &str) -> Result<Event, Error> {
        let nostr_event = nostr_sdk::Event::from_json(json).map_err(|e| Error::from(e))?;
        _ = nostr_event.verify().map_err(|e| Error::from(e))?;

        Event::try_from_json_str(nostr_event.content())
    }
}

/// Nostr event that pledges the event signer will make an [EventPayout] for a specific [Event] in the future.
pub struct FutureEventPayoutAttestationPledge;

impl FutureEventPayoutAttestationPledge {
    pub const NOSTR_KIND: Kind = Kind::Custom(6276);

    /// Creates [FutureEventPayoutAttestationPledge] nostr event json
    pub fn create_nostr_event_json(event: &Event, secret_key: &str) -> Result<String, Error> {
        let e_hash_hex = event.hash_hex()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(e_hash_hex.0.clone()).into()];

        let builder = nostr_sdk::EventBuilder::new(Self::NOSTR_KIND, e_hash_hex.0, tags);

        let keys = Keys::parse(secret_key).map_err(|e| Error::from(e))?;
        let event = builder.to_event(&keys).map_err(|e| Error::from(e))?;

        event.try_as_json().map_err(|e| Error::from(e))
    }

    /// Returns nostr public key hex and the hex hash of the [Event] it pledges to make a payout attestation to.
    pub fn interpret_nostr_event_json(
        json: &str,
    ) -> Result<(NostrPublicKeyHex, EventHashHex), Error> {
        let nostr_event = nostr_sdk::Event::from_json(json).map_err(|e| Error::from(e))?;
        _ = nostr_event.verify().map_err(|e| Error::from(e))?;

        let nostr_public_key_hex = nostr_event.pubkey.to_hex();
        let content = nostr_event.content().to_string();
        if !EventHashHex::is_hash_hex(&content) {
            return Err(Error::Validation(format!(
                "nostr event content does not have format of event hash hex"
            )));
        }

        Ok((
            NostrPublicKeyHex(nostr_public_key_hex),
            EventHashHex(content),
        ))
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
    ) -> Result<String, Error> {
        let event_payout_json = event_payout.try_to_json_string()?;
        let tags: Vec<Tag> =
            vec![TagStandard::Hashtag(event_payout.event_hash_hex.0.clone()).into()];

        let builder = nostr_sdk::EventBuilder::new(Self::NOSTR_KIND, event_payout_json, tags);

        let keys = Keys::parse(secret_key)
            .map_err(|e| Error::from(e))?;
        let event = builder
            .to_event(&keys)
            .map_err(|e| Error::from(e))?;

        event
            .try_as_json()
            .map_err(|e| Error::from(e))
    }

    /// Returns serialized nostr public key and the [EventPayout] it signed.
    /// IMPORTANT: EventPayout is not validated.
    pub fn interpret_nostr_event_json(
        json: &str,
    ) -> Result<(NostrPublicKeyHex, EventPayout), Error> {
        let nostr_event = nostr_sdk::Event::from_json(json)
            .map_err(|e| Error::from(e))?;
        _ = nostr_event
            .verify()
            .map_err(|e| Error::from(e))?;

        let nostr_public_key_hex = nostr_event.pubkey.to_hex();
        let event_payout = EventPayout::try_from_json_str(nostr_event.content())?;

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
