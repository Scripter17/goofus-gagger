//! Gags.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serenity::all::Timestamp;
use serenity::model::id::{GuildId, ChannelId, UserId};

use crate::types::*;
use crate::util::*;

/// A new [`Gag`] to apply to someone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewGag {
    /// The [`ChannelId`] to apply [`Self::gag`] in.
    pub channel: ChannelId,
    /// The gag to apply.
    pub gag: Gag
}

impl From<NewGag> for Gag {
    fn from(value: NewGag) -> Self {
        value.gag
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
    /// The config of a [`Gag`].
    #[serde(flatten)]
    pub config: GagConfig
}

/// The config of a [`Gag`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GagConfig {
    /// If [`true`], the [`Gaggee`] can't ungag themself and anyone trying to ungag them needs [`Trust::untie`] consent.
    #[serde(default)]
    pub tie: bool,
    /// The [`GagMode`] this gag uses.
    pub mode: GagModeName
}

/// Configures the default values for gags.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GagDefaults {
    /// The global defaults.
    #[serde(default, skip_serializing_if = "is_default")]
    pub global: GagConfigDiff,
    /// The per-server defaults. Overrides [`Self::global`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_guild: HashMap<GuildId, GagConfigDiff>,
    /// The per-user defaults. Overrides [`Self::per_guild`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_user: HashMap<UserId, GagConfigDiff>,
    /// The per-member defaults. Overrides [`Self::per_member`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_member: HashMap<MemberId, GagConfigDiff>
}

impl GagDefaults {
    /// Default values for a [`Gag`] for a [`MemberId`].
    pub fn default_for(&self, gagger: MemberId) -> GagConfig {
        let mut ret = Default::default();
        self.global.apply(&mut ret);
        if let Some(diff) = self.per_guild .get(&gagger.guild) {diff.apply(&mut ret);}
        if let Some(diff) = self.per_user  .get(&gagger.user ) {diff.apply(&mut ret);}
        if let Some(diff) = self.per_member.get(&gagger      ) {diff.apply(&mut ret);}
        ret
    }
}

/// Overrides for [`GagConfig`]s.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GagConfigDiff {
    /// If [`Some`], overwrites [`GagConfig::tie`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub tie: Option<bool>,
    /// If [`Some`], overwrites [`GagConfig::mode`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub mode: Option<GagModeName>
}

impl GagConfigDiff {
    /// Apply the diffs.
    pub fn apply(&self, to: &mut GagConfig) {
        if let Some(tie ) = self.tie  {to.tie  = tie ;}
        if let Some(mode) = self.mode {to.mode = mode;}
    }
}
