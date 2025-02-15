use serde::{Serialize, Deserialize};
use serenity::all::Timestamp;

/// A gag
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gag {
    /// The point in time where the gag no longer applies.
    pub until: GagUntil,
    /// If [`true`], the [`Gagee`] can't ungag themself and anyone trying to ungag them needs [`Trust::untie`] consent.
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

impl GagUntil {
    /// Basically [`Option::is_none_or`].
    pub fn is_forever_or<F: FnOnce(&Timestamp) -> bool>(&self, f: F) -> bool {
        match self {
            GagUntil::Time(x) => f(x),
            GagUntil::Forever => true
        }
    }
}
