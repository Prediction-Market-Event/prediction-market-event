use information::Information;
use rand::random;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod information;
pub mod nostr;

mod tests;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Event {
    pub nonce: [u8; 32],
    pub outcome_count: Outcome,
    pub units_to_payout: PayoutUnit,
    pub information: Information,
}

impl Event {
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

    pub fn try_to_json_string(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("failed event conversion to json: {e}"))
    }

    pub fn try_from_json_str(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("failed event conversion from json: {e}"))
    }

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

    pub fn hash_sha256(&self) -> Result<[u8; 32], String> {
        let mut hasher = Sha256::new();

        let json = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        hasher.update(json.as_slice());

        let mut out = [0u8; 32];
        hasher.finalize_into((&mut out).into());

        Ok(out)
    }

    pub fn hash_sha256_hex(&self) -> Result<String, String> {
        let a = self.hash_sha256()?;

        Ok(format!("{:02x?}", a))
    }
}

pub type Outcome = u16;

pub type PayoutUnit = u32;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EventPayout {
    pub event_hash_hex: String,
    pub units_per_outcome: Vec<PayoutUnit>,
}

impl EventPayout {
    pub fn new(event: &Event, payout: Vec<PayoutUnit>) -> Result<Self, String> {
        let event_hash_hex = event
            .hash_sha256_hex()
            .map_err(|e| format!("failed to get event hash hex: {e}"))?;

        Ok(Self {
            event_hash_hex,
            units_per_outcome: payout,
        })
    }

    pub fn try_to_json_string(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("failed event payout conversion to json: {e}"))
    }

    pub fn try_from_json_str(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("failed event payout conversion from json: {e}"))
    }

    pub fn validate(&self, event: &Event) -> Result<(), String> {
        let event_hash_hex = event
            .hash_sha256_hex()
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
