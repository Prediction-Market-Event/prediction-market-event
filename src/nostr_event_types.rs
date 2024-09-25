use std::fmt::Display;

use nostr::key::Keys;
#[allow(unused_imports)]
use nostr::{
    key::PublicKey, Event as NostrEvent, EventBuilder as NostrEventBuilder, Filter, JsonUtil, Kind, Tag, TagStandard,
    UnsignedEvent as NostrUnsignedEvent,
};
use serde::{Deserialize, Serialize};

use crate::{Error, Event as PredictionMarketEvent, EventHashHex, EventPayout, PayoutUnit};

/// [NostrEvent] containing a [PredictionMarketEvent]
pub struct NewEvent;

impl NewEvent {
    pub const NOSTR_KIND: u16 = 6275;

    /// Creates [NostrEventBuilder] with [NewEvent] parameters
    pub fn create_nostr_event_builder(
        event: &PredictionMarketEvent,
    ) -> Result<NostrEventBuilder, Error> {
        let event_json = event.try_to_json_string()?;
        let event_hash_hex = event.hash_hex()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_hash_hex.0).into()];
        let builder = NostrEventBuilder::new(Kind::from_u16(Self::NOSTR_KIND), event_json, tags);

        Ok(builder)
    }

    /// Creates [NewEvent] [NostrUnsignedEvent] json string
    pub fn create_nostr_unsigned_event_json(
        event: &PredictionMarketEvent,
        public_key: &str,
    ) -> Result<String, Error> {
        let public_key = PublicKey::parse(public_key)?;
        let builder = Self::create_nostr_event_builder(event)?;
        let nostr_unsigned_event = builder.to_unsigned_event(public_key);
        let nostr_unsigned_event_json = nostr_unsigned_event.try_as_json()?;

        Ok(nostr_unsigned_event_json)
    }

    /// Creates [NewEvent] [NostrEvent] json string
    pub fn create_nostr_signed_event_json(
        event: &PredictionMarketEvent,
        secret_key: &str,
    ) -> Result<String, Error> {
        let keys = Keys::parse(secret_key)?;
        let builder = Self::create_nostr_event_builder(event)?;
        let nostr_event = builder.to_event(&keys)?;
        let nostr_event_json = nostr_event.try_as_json()?;

        Ok(nostr_event_json)
    }

    /// Returns the [PredictionMarketEvent] found in nostr event.
    /// IMPORTANT: the returned [PredictionMarketEvent] is not validated.
    pub fn interpret_nostr_event_json(json: &str) -> Result<PredictionMarketEvent, Error> {
        let nostr_event = NostrEvent::from_json(json)?;
        nostr_event.verify()?;

        let event = PredictionMarketEvent::try_from_json_str(&nostr_event.content)?;

        let Some(hash_tag) = nostr_event.hashtags().next().map(|s| s.to_owned()) else {
            return Err(Error::Validation(format!(
                "nostr event does not have any hash tags"
            )));
        };
        if hash_tag != event.hash_hex()?.0 {
            return Err(Error::Validation(format!(
                "nostr event hash tag does not equal hash hex of contained event"
            )));
        }

        Ok(event)
    }

    /// Returns [Filter] as json that specifies kind [NewEvent]
    ///
    /// A [nostr::TagStandard::Hashtag] containing [PredictionMarketEvent::hash_hex] can
    /// be added to this filter to lookup an event by its hash hex.
    pub fn filter_json() -> String {
        Filter::new()
            .kind(Kind::from_u16(Self::NOSTR_KIND))
            .try_as_json()
            .unwrap()
    }
}

/// [NostrEvent] that pledges the signer will make an [EventPayoutAttestation] for a specific [PredictionMarketEvent] in the future.
pub struct FutureEventPayoutAttestationPledge;

impl FutureEventPayoutAttestationPledge {
    pub const NOSTR_KIND: u16 = 6276;

    /// Creates [NostrEventBuilder] with [FutureEventPayoutAttestationPledge] parameters
    pub fn create_nostr_event_builder(
        event_hash_hex: EventHashHex,
    ) -> Result<NostrEventBuilder, Error> {
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_hash_hex.0).into()];
        let builder = NostrEventBuilder::new(Kind::from_u16(Self::NOSTR_KIND), "", tags);

        Ok(builder)
    }

    /// Creates [FutureEventPayoutAttestationPledge] [NostrUnsignedEvent] json string
    pub fn create_nostr_unsigned_event_json(
        event_hash_hex: EventHashHex,
        public_key: &str,
    ) -> Result<String, Error> {
        let public_key = PublicKey::parse(public_key)?;
        let builder = Self::create_nostr_event_builder(event_hash_hex)?;
        let nostr_unsigned_event = builder.to_unsigned_event(public_key);
        let nostr_unsigned_event_json = nostr_unsigned_event.try_as_json()?;

        Ok(nostr_unsigned_event_json)
    }

    /// Creates [FutureEventPayoutAttestationPledge] [NostrEvent] json string
    pub fn create_nostr_signed_event_json(
        event_hash_hex: EventHashHex,
        secret_key: &str,
    ) -> Result<String, Error> {
        let keys = Keys::parse(secret_key)?;
        let builder = Self::create_nostr_event_builder(event_hash_hex)?;
        let nostr_event = builder.to_event(&keys)?;
        let nostr_event_json = nostr_event.try_as_json()?;

        Ok(nostr_event_json)
    }

    /// Returns [NostrPublicKeyHex] and the [EventHashHex] it pledges to make a [EventPayoutAttestation] to.
    pub fn interpret_nostr_event_json(
        json: &str,
    ) -> Result<(NostrPublicKeyHex, EventHashHex), Error> {
        let nostr_event = NostrEvent::from_json(json)?;
        nostr_event.verify()?;

        let nostr_public_key_hex = NostrPublicKeyHex(nostr_event.pubkey.to_hex());
        let Some(hash_tag) = nostr_event.hashtags().next().map(|s| s.to_owned()) else {
            return Err(Error::Validation(format!(
                "nostr event does not have any hash tags"
            )));
        };
        let Some(event_hash_hex) = EventHashHex::new_checked(&hash_tag) else {
            return Err(Error::Validation(format!(
                "nostr event hash tag does not have format of event hash hex"
            )));
        };

        Ok((nostr_public_key_hex, event_hash_hex))
    }

    /// Returns [Filter] as json that specifies kind [FutureEventPayoutAttestationPledge]
    ///
    /// A [nostr::TagStandard::Hashtag] containing [PredictionMarketEvent::hash_hex] can be
    /// added to this filter to lookup future attestation pledges relating to a certain [PredictionMarketEvent].
    pub fn filter_json() -> String {
        Filter::new()
            .kind(Kind::from_u16(Self::NOSTR_KIND))
            .try_as_json()
            .unwrap()
    }
}

/// [NostrEvent] that contains an [EventPayout] attestation
pub struct EventPayoutAttestation;

impl EventPayoutAttestation {
    pub const NOSTR_KIND: u16 = 6277;

    /// Creates [NostrEventBuilder] with [EventPayoutAttestation] parameters
    pub fn create_nostr_event_builder(
        event_payout: &EventPayout,
    ) -> Result<NostrEventBuilder, Error> {
        let units_per_outcome_json = serde_json::to_string(&event_payout.units_per_outcome)?;
        let tags: Vec<Tag> =
            vec![TagStandard::Hashtag(event_payout.event_hash_hex.0.clone()).into()];
        let builder = NostrEventBuilder::new(
            Kind::from_u16(Self::NOSTR_KIND),
            units_per_outcome_json,
            tags,
        );

        Ok(builder)
    }

    /// Creates [EventPayoutAttestation] [NostrUnsignedEvent] json string
    pub fn create_nostr_unsigned_event_json(
        event_payout: &EventPayout,
        public_key: &str,
    ) -> Result<String, Error> {
        let public_key = PublicKey::parse(public_key)?;
        let builder = Self::create_nostr_event_builder(event_payout)?;
        let nostr_unsigned_event = builder.to_unsigned_event(public_key);
        let nostr_unsigned_event_json = nostr_unsigned_event.try_as_json()?;

        Ok(nostr_unsigned_event_json)
    }

    /// Creates [EventPayoutAttestation] [NostrEvent] json string
    pub fn create_nostr_signed_event_json(
        event_payout: &EventPayout,
        secret_key: &str,
    ) -> Result<String, Error> {
        let keys = Keys::parse(secret_key)?;
        let builder = Self::create_nostr_event_builder(event_payout)?;
        let nostr_event = builder.to_event(&keys)?;
        let nostr_event_json = nostr_event.try_as_json()?;

        Ok(nostr_event_json)
    }

    /// Returns [NostrPublicKeyHex] and the [EventPayout] it signed.
    /// IMPORTANT: [EventPayout] is not validated.
    pub fn interpret_nostr_event_json(
        json: &str,
    ) -> Result<(NostrPublicKeyHex, EventPayout), Error> {
        let nostr_event = NostrEvent::from_json(json)?;
        nostr_event.verify()?;

        let nostr_public_key_hex = NostrPublicKeyHex(nostr_event.pubkey.to_hex());
        let Some(hash_tag) = nostr_event.hashtags().next().map(|s| s.to_owned()) else {
            return Err(Error::Validation(format!(
                "nostr event does not have any hash tags"
            )));
        };
        let Some(event_hash_hex) = EventHashHex::new_checked(&hash_tag) else {
            return Err(Error::Validation(format!(
                "nostr event hash tag does not have format of event hash hex"
            )));
        };
        let units_per_outcome: Vec<PayoutUnit> = serde_json::from_str(&nostr_event.content)?;
        let event_payout = EventPayout {
            event_hash_hex,
            units_per_outcome,
        };

        Ok((nostr_public_key_hex, event_payout))
    }

    /// Returns [Filter] as json that specifies kind [EventPayoutAttestation]
    ///
    /// A [nostr::TagStandard::Hashtag] containing [PredictionMarketEvent::hash_hex] can be
    /// added to this filter to lookup attestations relating to a certain [PredictionMarketEvent].
    pub fn filter_json() -> String {
        Filter::new()
            .kind(Kind::from_u16(Self::NOSTR_KIND))
            .try_as_json()
            .unwrap()
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
    /// Returns [Some] when s passes [Self::is_valid_format]
    pub fn new_checked(s: &str) -> Option<Self> {
        if Self::is_valid_format(s) {
            Some(Self(s.to_owned()))
        } else {
            None
        }
    }
}