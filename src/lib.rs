use std::fmt::Display;

use information::Information;
use rand::random;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod information;
pub mod nostr;
mod tests;

/// Prediction market event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Event {
    /// Randomness to ensure that unique events can be created easily.
    pub nonce: [u8; 32],

    /// How many different outcomes does this event have.
    pub outcome_count: Outcome,

    /// How many units can be used to make a payout to the outcomes.
    pub units_to_payout: PayoutUnit,

    /// Information about what this event is actually about.
    pub information: Information,
}

impl Event {
    /// Create new [Event]. [Event] is not validated.
    pub fn new_with_random_nonce(
        outcome_count: Outcome,
        units_to_payout: PayoutUnit,
        information: Information,
    ) -> Self {
        Self {
            nonce: random(),
            outcome_count,
            units_to_payout,
            information,
        }
    }

    /// Try to create json string from [Event]
    pub fn try_to_json_string(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("failed event conversion to json: {e}"))
    }

    /// Try to parse json string into [Event]. [Event] is not validated.
    pub fn try_from_json_str(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("failed event conversion from json: {e}"))
    }

    /// Validate [Event].
    /// accepted_information_variant_ids can be set to [Information::ALL_VARIANT_IDS] to accept any information variant.
    pub fn validate(&self, accepted_information_variant_ids: &[&str]) -> Result<(), String> {
        if self.outcome_count < 2 {
            return Err(format!("outcome count must be greater than 1"));
        }
        if self.units_to_payout < 1 {
            return Err(format!("units to payout must be greater than 0"));
        }
        if let Err(e) = self.information.validate(
            accepted_information_variant_ids,
            self.outcome_count,
            self.units_to_payout,
        ) {
            return Err(format!("failed to validate event information: {e}"));
        }

        Ok(())
    }

    /// Get sha256 hex hash of [Event]. This should be used for identifying this event and integrity checking.
    pub fn hash_hex(&self) -> Result<EventHashHex, String> {
        let hash = self.hash_sha256()?;

        let mut hash_hex = String::with_capacity(64);
        for b in hash {
            hash_hex.push_str(&format!("{b:02x}"))
        }

        Ok(EventHashHex(hash_hex))
    }

    /// internal sha256 hash
    fn hash_sha256(&self) -> Result<[u8; 32], String> {
        let mut hasher = Sha256::new();

        let json = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        hasher.update(json.as_slice());

        let mut out = [0u8; 32];
        hasher.finalize_into((&mut out).into());

        Ok(out)
    }
}

/// Outcome id type for [Event]
pub type Outcome = u16;

/// Payout unit type for [Event]
pub type PayoutUnit = u32;

/// Clarity struct for cleaner data handling.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EventHashHex(pub String);

impl Display for EventHashHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl EventHashHex {
    /// Checks if s has structure of event hex hash.
    pub fn is_hash_hex(s: &str) -> bool {
        s.len() == 64 && matches!(s.find(|c: char| !c.is_ascii_hexdigit()), None)
    }
}

/// Describes a payout for a certain event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EventPayout {
    /// Created from [Event::hash_hex]
    pub event_hash_hex: EventHashHex,

    /// How [Event::units_to_payout] should be distributed to the outcomes.
    /// Length should be [Event::outcome_count]
    pub units_per_outcome: Vec<PayoutUnit>,
}

impl EventPayout {
    /// Create new [EventPayout]. [EventPayout] is not validated.
    pub fn new(event: &Event, payout: Vec<PayoutUnit>) -> Result<Self, String> {
        let event_hash_hex = event
            .hash_hex()
            .map_err(|e| format!("failed to get event hash hex: {e}"))?;

        Ok(Self {
            event_hash_hex,
            units_per_outcome: payout,
        })
    }

    /// Try to create json string from [EventPayout]
    pub fn try_to_json_string(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("failed event payout conversion to json: {e}"))
    }

    /// Try to parse json string into [EventPayout]. [EventPayout] is not validated.
    pub fn try_from_json_str(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("failed event payout conversion from json: {e}"))
    }

    /// Validate [EventPayout]
    pub fn validate(&self, event: &Event) -> Result<(), String> {
        let event_hash_hex = event
            .hash_hex()
            .map_err(|e| format!("failed to get event hash hex: {e}"))?;
        if self.event_hash_hex != event_hash_hex {
            return Err(format!("event hashes do not match"));
        }

        if self.units_per_outcome.len() != usize::from(event.outcome_count) {
            return Err(format!(
                "event outcome count does not match number of items in payout vec"
            ));
        }

        let mut units_sum: PayoutUnit = 0;
        for u in self.units_per_outcome.iter() {
            units_sum = units_sum.checked_add(*u).ok_or(format!("overflow error"))?;
        }
        if event.units_to_payout != units_sum {
            return Err(format!(
                "unit sum of payout does not equal units available to payout according to event"
            ));
        }

        Ok(())
    }
}
