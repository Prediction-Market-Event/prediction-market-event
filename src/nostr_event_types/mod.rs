use crate::{Error, Event as PredictionMarketEvent, EventHashHex, EventPayout, PayoutUnit};
#[allow(unused_imports)]
use nostr::{
    key::PublicKey, Event as NostrEvent, EventBuilder as NostrEventBuilder, Filter, JsonUtil, Kind,
    Tag, TagStandard, UnsignedEvent as NostrUnsignedEvent,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use trait_dec::Res;

mod trait_dec;
pub use trait_dec::NostrEventUtils;

/// [NostrEvent] containing a [PredictionMarketEvent]
/// 
/// - kind set to [NewEvent::KIND]
/// - content set to [PredictionMarketEvent] as json.
/// - hashtag containing [PredictionMarketEvent::hash_hex]
pub struct NewEvent;

impl NostrEventUtils for NewEvent {
    const KIND_U16: u16 = 6275;

    type CreateParameter = PredictionMarketEvent;

    /// Accepts [PredictionMarketEvent]
    ///
    /// Returns [NostrEventBuilder] with:
    /// - kind set to [NewEvent::KIND]
    /// - content set to [PredictionMarketEvent] as json.
    /// - hashtag containing [PredictionMarketEvent::hash_hex]
    fn create_nostr_event_builder(event: &Self::CreateParameter) -> Res<NostrEventBuilder> {
        let event_json = event.try_to_json_string()?;
        let event_hash_hex = event.hash_hex()?;
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_hash_hex.0).into()];
        let builder = NostrEventBuilder::new(Self::KIND, event_json, tags);

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
}

/// [NostrEvent] that pledges the signer will make an [EventPayoutAttestation] for a specific [PredictionMarketEvent] in the future.
/// 
/// - kind set to [FutureEventPayoutAttestationPledge::KIND]
/// - content is empty.
/// - hashtag containing [EventHashHex]
pub struct FutureEventPayoutAttestationPledge;

impl NostrEventUtils for FutureEventPayoutAttestationPledge {
    const KIND_U16: u16 = 6276;

    type CreateParameter = EventHashHex;

    /// Accepts [EventHashHex]
    ///
    /// Returns [NostrEventBuilder] with:
    /// - kind set to [FutureEventPayoutAttestationPledge::KIND]
    /// - content is empty.
    /// - hashtag containing [EventHashHex]
    fn create_nostr_event_builder(
        event_hash_hex: &Self::CreateParameter,
    ) -> Res<NostrEventBuilder> {
        let tags: Vec<Tag> = vec![TagStandard::Hashtag(event_hash_hex.0.to_owned()).into()];
        let builder = NostrEventBuilder::new(Self::KIND, "", tags);

        Ok(builder)
    }

    type InterpretResult = (NostrPublicKeyHex, EventHashHex);

    /// Accepts [NostrEvent].
    ///
    /// Returns [NostrPublicKeyHex] and the [EventHashHex] it pledges to make a [EventPayoutAttestation] to.
    fn interpret_nostr_event(nostr_event: &NostrEvent) -> Res<Self::InterpretResult> {
        nostr_event.verify()?;

        let nostr_public_key_hex = NostrPublicKeyHex(nostr_event.pubkey.to_hex());
        let Some(hash_tag) = nostr_event.hashtags().next().map(|s| s.to_owned()) else {
            return Err(Error::Validation(format!(
                "nostr event does not have any hash tags"
            )));
        };
        let event_hash_hex = EventHashHex::from_str(&hash_tag)?;

        Ok((nostr_public_key_hex, event_hash_hex))
    }
}

/// [NostrEvent] that contains an [EventPayout] attestation
///
/// - kind set to [EventPayoutAttestation::KIND]
/// - content set [EventPayout::units_per_outcome] as json
/// - hashtag containing [EventPayout::event_hash_hex]
pub struct EventPayoutAttestation;

impl NostrEventUtils for EventPayoutAttestation {
    const KIND_U16: u16 = 6277;

    type CreateParameter = EventPayout;

    /// Accepts [EventPayout]
    ///
    /// Returns [NostrEventBuilder] with:
    /// - kind set to [EventPayoutAttestation::KIND]
    /// - content set [EventPayout::units_per_outcome] as json
    /// - hashtag containing [EventPayout::event_hash_hex]
    fn create_nostr_event_builder(event_payout: &Self::CreateParameter) -> Res<NostrEventBuilder> {
        let units_per_outcome_json = serde_json::to_string(&event_payout.units_per_outcome)?;
        let tags: Vec<Tag> =
            vec![TagStandard::Hashtag(event_payout.event_hash_hex.0.clone()).into()];
        let builder = NostrEventBuilder::new(Self::KIND, units_per_outcome_json, tags);

        Ok(builder)
    }

    type InterpretResult = (NostrPublicKeyHex, EventPayout);

    /// Accepts [NostrEvent].
    ///
    /// Returns [NostrPublicKeyHex] and the [EventPayout] it signed.
    /// IMPORTANT: [EventPayout] is not validated.
    fn interpret_nostr_event(nostr_event: &NostrEvent) -> Res<Self::InterpretResult> {
        nostr_event.verify()?;

        let nostr_public_key_hex = NostrPublicKeyHex(nostr_event.pubkey.to_hex());
        let Some(hash_tag) = nostr_event.hashtags().next().map(|s| s.to_owned()) else {
            return Err(Error::Validation(format!(
                "nostr event does not have any hash tags"
            )));
        };
        let event_hash_hex = EventHashHex::from_str(&hash_tag)?;
        let units_per_outcome: Vec<PayoutUnit> = serde_json::from_str(&nostr_event.content)?;
        let event_payout = EventPayout {
            event_hash_hex,
            units_per_outcome,
        };

        Ok((nostr_public_key_hex, event_payout))
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

impl FromStr for NostrPublicKeyHex {  
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid_format(s) {
            Ok(Self(s.to_owned()))
        } else {
            Err(Error::Validation(format!("invalid format")))
        }
    }
}
