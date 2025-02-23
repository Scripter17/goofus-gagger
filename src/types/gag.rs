//! Gags.

use serde::{Serialize, Deserialize};
use serenity::all::Timestamp;
use serenity::model::id::ChannelId;

use crate::types::*;
use crate::util::*;

/// A new [`Gag`] to apply to someone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewGag {
    /// The [`ChannelId`] to apply it in.
    pub channel: ChannelId,
    /// [`Gag::until`].
    pub until: Option<Timestamp>,
    /// [`Gag::tie`].
    pub tie: bool,
    /// [`Gag::mode`]
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

/// A new... uh, ungagging?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewUngag {
    /// The channel to remove the gag from.
    pub channel: ChannelId
}

/// A gag
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gag {
    /// The point in time where the gag no longer applies.
    #[serde(default, skip_serializing_if = "is_default")]
    pub until: Option<Timestamp>,
    /// If [`true`], the [`Gaggee`] can't ungag themself and anyone trying to ungag them needs [`Trust::untie`] consent.
    #[serde(default)]
    pub tie: bool,
    /// The [`GagMode`] this gag uses.
    pub mode: GagModeName
}
