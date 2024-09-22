use std::fmt::Display;

use nostr_sdk::{Event as NostrEvent, EventBuilder, JsonUtil, Keys, Kind, Tag, TagStandard};
use serde::{Deserialize, Serialize};

use crate::{Error, Event as PredictionMarketEvent, EventHashHex, EventPayout};

/// [NostrEvent] containing a [PredictionMarketEvent]
pub struct NewEvent;

impl NewEvent {
    pub const NOSTR_KIND: Kind = Kind::Custom(6275);

    /// Creates [NewEvent] [NostrEvent] json string
    pub fn create_nostr_event_json(
        event: &PredictionMarketEvent,
        secret_key: &str,
    ) -> Result<String, Error> {
        let event_json = event.try_to_json_string()?;

        let event_hash_hex = event.hash_hex()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_hash_hex.0).into()];

        let builder = EventBuilder::new(Self::NOSTR_KIND, event_json, tags);

        let keys = Keys::parse(secret_key).map_err(|e| Error::from(e))?;
        let nostr_event = builder.to_event(&keys).map_err(|e| Error::from(e))?;

        nostr_event.try_as_json().map_err(|e| Error::from(e))
    }

    /// Returns the [PredictionMarketEvent] found in nostr event.
    /// IMPORTANT: the returned [PredictionMarketEvent] is not validated.
    pub fn interpret_nostr_event_json(json: &str) -> Result<PredictionMarketEvent, Error> {
        let nostr_event = NostrEvent::from_json(json).map_err(|e| Error::from(e))?;
        nostr_event.verify().map_err(|e| Error::from(e))?;

        PredictionMarketEvent::try_from_json_str(nostr_event.content())
    }
}

/// [NostrEvent] that pledges the signer will make an [EventPayoutAttestation] for a specific [PredictionMarketEvent] in the future.
pub struct FutureEventPayoutAttestationPledge;

impl FutureEventPayoutAttestationPledge {
    pub const NOSTR_KIND: Kind = Kind::Custom(6276);

    /// Creates [FutureEventPayoutAttestationPledge] [NostrEvent] json string
    pub fn create_nostr_event_json(
        event: &PredictionMarketEvent,
        secret_key: &str,
    ) -> Result<String, Error> {
        let event_hash_hex = event.hash_hex()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_hash_hex.0.clone()).into()];

        let builder = EventBuilder::new(Self::NOSTR_KIND, event_hash_hex.0, tags);

        let keys = Keys::parse(secret_key).map_err(|e| Error::from(e))?;
        let nostr_event = builder.to_event(&keys).map_err(|e| Error::from(e))?;

        nostr_event.try_as_json().map_err(|e| Error::from(e))
    }

    /// Returns [NostrPublicKeyHex] and the [EventHashHex] it pledges to make a [EventPayoutAttestation] to.
    pub fn interpret_nostr_event_json(
        json: &str,
    ) -> Result<(NostrPublicKeyHex, EventHashHex), Error> {
        let nostr_event = NostrEvent::from_json(json).map_err(|e| Error::from(e))?;
        nostr_event.verify().map_err(|e| Error::from(e))?;

        let nostr_public_key_hex = nostr_event.pubkey.to_hex();
        let content = nostr_event.content().to_string();
        if !EventHashHex::is_valid_format(&content) {
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

/// [NostrEvent] that contains an [EventPayout] attestation
pub struct EventPayoutAttestation;

impl EventPayoutAttestation {
    pub const NOSTR_KIND: Kind = Kind::Custom(6277);

    /// Creates [EventPayoutAttestation] [NostrEvent] json string
    pub fn create_nostr_event_json(
        event_payout: &EventPayout,
        secret_key: &str,
    ) -> Result<String, Error> {
        let event_payout_json = event_payout.try_to_json_string()?;
        let tags: Vec<Tag> =
            vec![TagStandard::Hashtag(event_payout.event_hash_hex.0.clone()).into()];

        let builder = EventBuilder::new(Self::NOSTR_KIND, event_payout_json, tags);

        let keys = Keys::parse(secret_key).map_err(|e| Error::from(e))?;
        let nostr_event = builder.to_event(&keys).map_err(|e| Error::from(e))?;

        nostr_event.try_as_json().map_err(|e| Error::from(e))
    }

    /// Returns [NostrPublicKeyHex] and the [EventPayout] it signed.
    /// IMPORTANT: [EventPayout] is not validated.
    pub fn interpret_nostr_event_json(
        json: &str,
    ) -> Result<(NostrPublicKeyHex, EventPayout), Error> {
        let nostr_event = NostrEvent::from_json(json).map_err(|e| Error::from(e))?;
        nostr_event.verify().map_err(|e| Error::from(e))?;

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

impl NostrPublicKeyHex {
    /// Checks if s has structure of nostr public key hex.
    pub fn is_valid_format(s: &str) -> bool {
        s.len() == 64 && matches!(s.find(|c: char| !c.is_ascii_hexdigit()), None)
    }
}
