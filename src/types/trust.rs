use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serenity::model::id::{UserId, GuildId};

use crate::util::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GageeTrust {
    #[serde(default, skip_serializing_if = "is_default")]
    pub default: Trust,
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_guild: HashMap<GuildId, GuildTrust>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_user: HashMap<UserId, GagerTrust>
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trust {
    #[serde(default, skip_serializing_if = "is_default")]
    pub gag: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub ungag: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub tie: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub untie: bool
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildTrust {
    pub default: TrustDiff
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GagerTrust {
    pub default: TrustDiff,
    #[serde(default, skip_serializing_if = "is_default")]
    pub per_guild: HashMap<GuildId, TrustDiff>
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustDiff {
    #[serde(default, skip_serializing_if = "is_default")]
    pub gag: Option<bool>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub ungag: Option<bool>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub tie: Option<bool>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub untie: Option<bool>
}

impl TrustDiff {
    pub fn apply(self, to: &mut Trust) {
        if let Some(x) = self.gag   {to.gag   = x;}
        if let Some(x) = self.ungag {to.ungag = x;}
        if let Some(x) = self.tie   {to.tie   = x;}
        if let Some(x) = self.untie {to.untie = x;}
    }
}
