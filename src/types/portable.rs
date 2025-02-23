//! A representation of a gaggee that can be exported and imported between bot instances.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serenity::model::id::ChannelId;

use crate::types::*;

/// A representation of a gaggee that can be exported and imported between bot instances.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableGaggee {
    /// [`State::trusts`].
    pub trusts: Option<GaggeeTrust>,
    /// [`State::gags`].
    pub gags: Option<HashMap<ChannelId, Gag>>,
    /// [`State::max_msg_lengths`].
    pub max_msg_length: Option<usize>,
    /// [`State::safewords`].
    pub safewords: Option<Safewords>
}
