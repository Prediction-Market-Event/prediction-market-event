use crate::{Outcome, PayoutUnit};
use serde::{Deserialize, Serialize};

pub mod information_variants;
pub use information_variants::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Information {
    Empty,
    V1(V1),
}

impl Information {
    pub fn information_variant_id(&self) -> &'static str {
        match self {
            Self::Empty => Empty::ID,
            Self::V1(_) => V1::ID,
        }
    }

    pub fn validate(
        &self,
        outcome_count: Outcome,
        _units_to_payout: PayoutUnit,
        accepted_information_variant_ids: &[&str],
    ) -> Result<(), String> {
        if !accepted_information_variant_ids.contains(&self.information_variant_id()) {
            return Err(format!("information variant id not accepted"));
        }

        match self {
            Self::Empty => Ok(()),
            Self::V1(i) => i.validate(outcome_count),
        }
    }
}
