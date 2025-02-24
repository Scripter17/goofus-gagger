//! A representation of a gaggee that can be exported and imported between bot instances.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serenity::model::id::ChannelId;

use crate::types::*;

/// A representation of a gaggee that can be exported and imported between bot instances.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableGaggee {
    /// [`State::trusts`].
    #[serde(default)]
    pub trusts: Option<GaggeeTrust>,
    /// [`State::gags`].
    #[serde(default)]
    pub gags: Option<HashMap<ChannelId, Gag>>,
    /// [`State::max_msg_lengths`].
    #[serde(default)]
    pub max_msg_length: Option<usize>,
    /// [`State::safewords`].
    #[serde(default)]
    pub safewords: Option<Safewords>,
    /// [`State::gag_defaults`],
    #[serde(default)]
    pub gag_defaults: Option<GagDefaults>
}
