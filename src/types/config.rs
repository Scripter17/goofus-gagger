//! Probably should merge this somewehre else.

use serde::{Serialize, Deserialize};

use crate::types::*;

/// What to do to a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageAction {
    /// Gag with with the specified [`GagMode`].
    Gag(GagModeName),
    /// Warn that it's too long to gag.
    WarnTooLong(usize)
}
