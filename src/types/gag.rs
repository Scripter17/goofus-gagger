use std::cmp::Ordering;

use serde::{Serialize, Deserialize};
use serenity::all::Timestamp;

/// A gag
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gag {
    /// The point in time where the gag no longer applies.
    pub until: GagUntil,
    /// If [`true`], the [`Gaggee`] can't ungag themself and anyone trying to ungag them needs [`Trust::untie`] consent.
    #[serde(default)]
    pub tie: bool
}

/// The point in time where a gag no longer applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum GagUntil {
    /// A specific timestamp.
    Time(Timestamp),
    /// Makes the gag apply until removed.
    Forever
}

impl PartialEq<Timestamp> for GagUntil {
    fn eq(&self, other: &Timestamp) -> bool {
        match self {
            Self::Time(x) => x == other,
            Self::Forever => false
        }
    }
}

impl PartialOrd<Timestamp> for GagUntil {
    fn partial_cmp(&self, other: &Timestamp) -> Option<Ordering> {
        match self {
            Self::Time(x) => x.partial_cmp(other),
            Self::Forever => Some(Ordering::Less)
        }
    }
}
