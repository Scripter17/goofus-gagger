use serde::{Serialize, Deserialize};
use serenity::all::Timestamp;
use serenity::model::id::ChannelId;

use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewGag {
    pub channel: ChannelId,
    pub until: Option<Timestamp>,
    pub tie: bool,
    pub mode: GagModeName
}

impl From<NewGag> for Gag {
    fn from(value: NewGag) -> Self {
        Self {
            until: value.until,
            tie: value.tie,
            mode: value.mode
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewUngag {
    pub channel: ChannelId
}

/// A gag
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gag {
    /// The point in time where the gag no longer applies.
    pub until: Option<Timestamp>,
    /// If [`true`], the [`Gaggee`] can't ungag themself and anyone trying to ungag them needs [`Trust::untie`] consent.
    #[serde(default)]
    pub tie: bool,
    pub mode: GagModeName
}
