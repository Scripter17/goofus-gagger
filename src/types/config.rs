use serde::{Serialize, Deserialize};

use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageAction {
    Gag(GagModeName),
    WarnTooLong(usize)
}
