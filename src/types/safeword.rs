use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use serenity::model::id::{GuildId, ChannelId};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Safewords {
    pub global: bool,
    pub servers: HashSet<GuildId>,
    pub channels: HashSet<ChannelId>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, poise::ChoiceParameter)]
pub enum SafewordLocation {
    Global,
    Server,
    Channel
}

pub enum SafewordError {
    NotInServer
}

impl Safewords {
    pub fn add_safeword(&mut self, location: SafewordLocation, channel: ChannelId, server: Option<GuildId>) -> Result<bool, SafewordError> {
        Ok(match location {
            SafewordLocation::Global => {let ret = !self.global; self.global = true; ret},
            SafewordLocation::Server => self.servers.insert(server.ok_or(SafewordError::NotInServer)?),
            SafewordLocation::Channel => self.channels.insert(channel)
        })
    }

    pub fn remove_safeword(&mut self, location: SafewordLocation, channel: ChannelId, server: Option<GuildId>) -> Result<bool, SafewordError> {
        Ok(match location {
            SafewordLocation::Global => {let ret = self.global; self.global=false; ret},
            SafewordLocation::Server => self.servers.remove(&server.ok_or(SafewordError::NotInServer)?),
            SafewordLocation::Channel => self.channels.remove(&channel)
        })
    }

    pub fn is_safewording(&self, channel: ChannelId, server: Option<GuildId>) -> bool {
        self.global || server.is_some_and(|server| self.servers.contains(&server)) || self.channels.contains(&channel)
    }
}
