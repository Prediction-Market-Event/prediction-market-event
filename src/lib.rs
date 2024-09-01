use information::Information;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod information;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Event {
    pub nonce: [u8; 32],
    pub outcome_count: Outcome,
    pub units_to_payout: UnitsToPayout,
    pub information: Information,
}

impl Event {
    pub fn validate(&self) -> Result<(), String> {
        if let Err(e) = self
            .information
            .validate(self.outcome_count, self.units_to_payout)
        {
            return Err(format!("failed to validate event information: {e}"));
        }

        Ok(())
    }

    pub fn hash_sha256<'a>(&self) -> Result<[u8; 32], String> {
        let mut hasher = Sha256::new();

        let json = serde_json::to_string(self).map_err(|e| e.to_string())?;
        hasher.update(json.as_bytes());

        let mut out = [0u8; 32];
        hasher.finalize_into((&mut out).into());

        Ok(out)
    }
}

pub type Outcome = u16;

pub type UnitsToPayout = u32;
