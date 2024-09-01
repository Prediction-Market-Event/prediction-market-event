use serde::{Deserialize, Serialize};

use crate::{Outcome, UnitsToPayout};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Information {
    None,
    V1(V1),
}

impl Information {
    pub fn validate(
        &self,
        outcome_count: Outcome,
        _units_to_payout: UnitsToPayout,
    ) -> Result<(), String> {
        match self {
            Self::None => Ok(()),
            Self::V1(i) => i.validate(outcome_count),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V1 {
    pub title: String,
    pub description: String,
    pub outcome_titles: Vec<String>,
    pub expected_payout_unix_seconds: u64,
}

impl V1 {
    // hard coded string length limits
    const MAX_TITLE_LENGTH: usize = 256;
    const MAX_DESCRIPTION_LENGTH: usize = 1024 * 10;
    const MAX_OUTCOME_TITLE_LENGTH: usize = 64;

    pub fn validate(&self, outcomes: Outcome) -> Result<(), String> {
        if self.title.len() > Self::MAX_TITLE_LENGTH {
            return Err(format!("title length is over limit"));
        }
        if self.description.len() > Self::MAX_DESCRIPTION_LENGTH {
            return Err(format!("description length is over max"));
        }
        if self.outcome_titles.len() != usize::from(outcomes) {
            return Err(format!(
                "outcome titles array length does not equal number of outcomes"
            ));
        }

        for (i, outcome_title) in self.outcome_titles.iter().enumerate() {
            if outcome_title.len() > Self::MAX_OUTCOME_TITLE_LENGTH {
                return Err(format!("outcome {i}: title length is over max"));
            }
        }

        Ok(())
    }
}