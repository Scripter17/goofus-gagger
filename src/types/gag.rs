use serde::{Serialize, Deserialize};
use serenity::all::Timestamp;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gag {
    pub until: GagUntil,
    #[serde(default)]
    pub tie: bool
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum GagUntil {
    Time(Timestamp),
    Forever
}

impl GagUntil {
    pub fn is_forever_or<F: FnOnce(&Timestamp) -> bool>(&self, f: F) -> bool {
        match self {
            GagUntil::Time(x) => f(x),
            GagUntil::Forever => true
        }
    }
}
