use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serenity::model::id::ChannelId;

use crate::types::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableGaggee {
    pub trusts: Option<GaggeeTrust>,
    pub gags: Option<HashMap<ChannelId, Gag>>,
    pub max_msg_length: Option<usize>,
    pub safewords: Option<Safewords>
}
