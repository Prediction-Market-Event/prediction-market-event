use serde::{Deserialize, Serialize};
use trait_dec::Res;
use std::fmt::Display;
use crate::{Error, Event as PredictionMarketEvent, EventHashHex, EventPayout, PayoutUnit};
#[allow(unused_imports)]
use nostr::{
    key::PublicKey, Event as NostrEvent, EventBuilder as NostrEventBuilder, Filter, JsonUtil, Kind,
    Tag, TagStandard, UnsignedEvent as NostrUnsignedEvent,
};

mod trait_dec;
pub use trait_dec::NostrEventUtils;


/// [NostrEvent] containing a [PredictionMarketEvent]
pub struct NewEvent;

impl NostrEventUtils for NewEvent {
    const NOSTR_KIND: u16 = 6275;

    type CreateParameter = PredictionMarketEvent;

    fn create_nostr_event_builder(
        event: &Self::CreateParameter,
    ) -> Res<NostrEventBuilder> {
        let event_json = event.try_to_json_string()?;
        let event_hash_hex = event.hash_hex()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_hash_hex.0).into()];
        let builder = NostrEventBuilder::new(Kind::from_u16(Self::NOSTR_KIND), event_json, tags);

        Ok(builder)
    }

    type InterpretResult = PredictionMarketEvent;

    /// Accepts [NostrEvent].
    ///
    /// Returns the [PredictionMarketEvent].
    /// IMPORTANT: the returned [PredictionMarketEvent] is not validated.
    fn interpret_nostr_event(nostr_event: &NostrEvent) -> Res<Self::InterpretResult> {
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

    /// Returns [Filter] that specifies kind [NewEvent]
    ///
    /// A [nostr::TagStandard::Hashtag] containing [PredictionMarketEvent::hash_hex] can
    /// be added to this filter to lookup a [PredictionMarketEvent] by its [EventHashHex].
    fn filter() -> Filter {
        Filter::new().kind(Kind::from_u16(Self::NOSTR_KIND))
    }
}

/// [NostrEvent] that pledges the signer will make an [EventPayoutAttestation] for a specific [PredictionMarketEvent] in the future.
pub struct FutureEventPayoutAttestationPledge;

impl NostrEventUtils for FutureEventPayoutAttestationPledge {
    const NOSTR_KIND: u16 = 6276;

    type CreateParameter = EventHashHex;

    fn create_nostr_event_builder(
        event_hash_hex: &Self::CreateParameter,
    ) -> Res<NostrEventBuilder> {
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_hash_hex.0.to_owned()).into()];
        let builder = NostrEventBuilder::new(Kind::from_u16(Self::NOSTR_KIND), "", tags);

        Ok(builder)
    }

    type InterpretResult = (NostrPublicKeyHex, EventHashHex);

    /// Accepts [NostrEvent].
    ///
    /// Returns [NostrPublicKeyHex] and the [EventHashHex] it pledges to make a [EventPayoutAttestation] to.
    fn interpret_nostr_event(
        nostr_event: &NostrEvent,
    ) -> Res<Self::InterpretResult> {
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

    /// Returns [Filter] that specifies kind [FutureEventPayoutAttestationPledge].
    ///
    /// A [nostr::TagStandard::Hashtag] containing [PredictionMarketEvent::hash_hex] can
    /// be added to this filter to lookup [FutureEventPayoutAttestationPledge] specifying to a certain [PredictionMarketEvent].
    fn filter() -> Filter {
        Filter::new().kind(Kind::from_u16(Self::NOSTR_KIND))
    }
}

/// [NostrEvent] that contains an [EventPayout] attestation
pub struct EventPayoutAttestation;

impl NostrEventUtils for EventPayoutAttestation {
    const NOSTR_KIND: u16 = 6277;

    type CreateParameter = EventPayout;

    /// Creates [NostrEventBuilder] with [EventPayoutAttestation] parameters
    fn create_nostr_event_builder(
        event_payout: &Self::CreateParameter,
    ) -> Res<NostrEventBuilder> {
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

    type InterpretResult = (NostrPublicKeyHex, EventPayout);

    /// Accepts [NostrEvent].
    ///
    /// Returns [NostrPublicKeyHex] and the [EventPayout] it signed.
    /// IMPORTANT: [EventPayout] is not validated.
    fn interpret_nostr_event(
        nostr_event: &NostrEvent,
    ) -> Res<Self::InterpretResult> {
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

    /// Returns [Filter] that specifies kind [EventPayoutAttestation]
    ///
    /// A [nostr::TagStandard::Hashtag] containing [PredictionMarketEvent::hash_hex] can be
    /// added to this filter to lookup [EventPayoutAttestation] specifying a certain [PredictionMarketEvent].
    fn filter() -> Filter {
        Filter::new().kind(Kind::from_u16(Self::NOSTR_KIND))
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
