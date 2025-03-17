//! The safeword system.

use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use serenity::model::id::{GuildId, ChannelId};

/// Configuration for where to ignore [`Gag`]s.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Safewords {
    /// If [`true`], all gags everywhere are safeworded.
    pub global: bool,
    /// The set of servers to safeword in.
    pub servers: HashSet<GuildId>,
    /// The set of channels to safeword in.
    pub channels: HashSet<ChannelId>
}

/// Command parameter to choose where to set/unset a safeword.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, poise::ChoiceParameter)]
pub enum SafewordLocation {
    /// [`Safeword::global`].
    Global,
    /// [`Safeword::servers`] with the current server.
    Server,
    /// [`Safeword::channels`] with the current channel.
    Channel
}

/// The enum of errors [`Safewords::add_safeword`] and [`Safewords::remove_safeword`] can reutrn.
pub enum SafewordError {
    /// Attempted to set/unset a server safeword when not in a server.
    NotInServer
}

impl Safewords {
    /// Set a safeword.
    /// # Errors
    /// If `location` is [`SafewordLocation::Server`] and `server` is [`None`], returns the error [`SafewordError::NotInServer`].
    pub fn add_safeword(&mut self, location: SafewordLocation, channel: ChannelId, server: Option<GuildId>) -> Result<bool, SafewordError> {
        Ok(match location {
            SafewordLocation::Global => {let ret = !self.global; self.global = true; ret},
            SafewordLocation::Server => self.servers.insert(server.ok_or(SafewordError::NotInServer)?),
            SafewordLocation::Channel => self.channels.insert(channel)
        })
    }

    /// Unset a safeword.
    /// # Errors
    /// If `location` is [`SafewordLocation::Server`] and `server` is [`None`], returns the error [`SafewordError::NotInServer`].
    pub fn remove_safeword(&mut self, location: SafewordLocation, channel: ChannelId, server: Option<GuildId>) -> Result<bool, SafewordError> {
        Ok(match location {
            SafewordLocation::Global => {let ret = self.global; self.global=false; ret},
            SafewordLocation::Server => self.servers.remove(&server.ok_or(SafewordError::NotInServer)?),
            SafewordLocation::Channel => self.channels.remove(&channel)
        })
    }

    /// Get the relevant safewords.
    ///
    /// Used by the `/status` command.
    pub fn get_relevant_safewords(&self, channel: ChannelId, server: Option<GuildId>) -> Vec<SafewordLocation> {
        let mut ret = Vec::new();
        if self.global {ret.push(SafewordLocation::Global);}
        if server.is_some_and(|server| self.servers.contains(&server)) {ret.push(SafewordLocation::Global);}
        if self.channels.contains(&channel) {ret.push(SafewordLocation::Channel);}
        ret
    }

    /// Checks if a channel is being safeworded.
    ///
    /// Let's see you include the server in that description with the proper hypothetical branch handling.
    pub fn is_safewording(&self, channel: ChannelId, server: Option<GuildId>) -> bool {
        self.global || server.is_some_and(|server| self.servers.contains(&server)) || self.channels.contains(&channel)
    }
}
