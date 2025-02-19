use std::collections::HashMap;
use std::sync::RwLock;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use std::collections::hash_map::Entry;

use serde::{Serialize, Deserialize};
use serenity::model::{channel::Message, id::{ChannelId, UserId, GuildId}, guild::Member};
use thiserror::Error;

use crate::util::*;

mod gag;
pub use gag::*;
mod trust;
pub use trust::*;
mod safeword;
pub use safeword::*;
mod gaggee;
pub use gaggee::*;
mod config;
pub use config::*;
mod member_id;
pub use member_id::*;

/// THe current state of the bot.
pub struct State {
    /// The config to use.
    pub config: RwLock<Config>,
    /// The path the config is stored at.
    pub path: PathBuf
}

#[derive(Debug, Error)]
pub enum WriteConfigError {
    #[error(transparent)] IoError(#[from] io::Error),
    #[error(transparent)] SerdeJsonError(#[from] serde_json::Error),
    #[error("A panic happened and the Config may be in an invalid state.")] PoisonError
}

impl<T> From<std::sync::PoisonError<T>> for WriteConfigError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::PoisonError
    }
}

impl State {
    /// Save the config to disk.
    pub fn write_to_file(&self) -> Result<(), WriteConfigError> {
        let new = serde_json::to_string_pretty(&*self.config.read()?)?;
        OpenOptions::new().write(true).truncate(true).open(&self.path)?.write_all(new.as_bytes())?;
        Ok(())
    }
}
