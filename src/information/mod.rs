use crate::*;
use serde::{Deserialize, Serialize};

pub mod information_variants;
pub use information_variants::*;

/// Different types of information an [Event] can have.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Information {
    None,
    V1(V1),
}

impl Information {
    /// Can be used in [Information::validate] to accept any information variant
    pub const ALL_VARIANT_IDS: &'static [&'static str] = &[None::ID, V1::ID];

    /// Get string id of information variant
    pub fn information_variant_id(&self) -> &'static str {
        match self {
            Self::None => None::ID,
            Self::V1(_) => V1::ID,
        }
    }

    /// Validate [Information]
    pub fn validate(
        &self,
        accepted_information_variant_ids: &[&str],
        outcome_count: Outcome,
        _units_to_payout: PayoutUnit,
    ) -> Result<(), Error> {
        if !accepted_information_variant_ids.contains(&self.information_variant_id()) {
            return Err(Error::Validation(format!(
                "information variant id not accepted"
            )));
        }

        match self {
            Self::None => Ok(()),
            Self::V1(i) => i.validate(outcome_count),
        }
    }
}
