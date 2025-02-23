use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use serenity::model::id::{UserId, GuildId};

use crate::types::*;
use crate::util::*;

/// The trusts a [`Gaggee`] has.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GaggeeTrust {
    /// the trust the gaggee has in servers.
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_guild: HashMap<GuildId, TrustDiff>,
    /// The trust the gaggee has in users.
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_user: HashMap<UserId, TrustDiff>,
    /// The trust the gaggee has in members.
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_member: HashMap<MemberId, TrustDiff>
}

/// The trust levels a server member has.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trust {
    /// Can gag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub gag: bool,
    /// Can ungag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ungag: bool,
    /// Can tie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tie: bool,
    /// Can untie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub untie: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub gag_modes: HashSet<GagModeName>
}

impl Trust {
    pub fn for_self() -> Self {
        Self {
            gag: true,
            ungag: true,
            tie: true,
            untie: false,
            gag_modes: GagModeName::all()
        }
    }
}

/// The overrides to apply to [`Trust`]s.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustDiff {
    /// If [`Some`], overwrites [`Trusr::gag`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub gag: Option<bool>,
    /// If [`Some`], overwrites [`Trusr::ungag`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub ungag: Option<bool>,
    /// If [`Some`], overwrites [`Trusr::tie`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub tie: Option<bool>,
    /// If [`Some`], overwrites [`Trusr::untie`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub untie: Option<bool>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub allow_gag_modes: HashSet<GagModeName>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub disallow_gag_modes: HashSet<GagModeName>
}

impl TrustDiff {
    /// Applies the overrides.
    pub fn apply(&self, to: &mut Trust) {
        if let Some(x) = self.gag   {to.gag   = x;}
        if let Some(x) = self.ungag {to.ungag = x;}
        if let Some(x) = self.tie   {to.tie   = x;}
        if let Some(x) = self.untie {to.untie = x;}
        for mode in &self.allow_gag_modes    {to.gag_modes.insert(*mode);}
        for mode in &self.disallow_gag_modes {to.gag_modes.remove( mode);}
    }
}
