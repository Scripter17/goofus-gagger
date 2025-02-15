use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use serenity::model::id::ChannelId;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafewordConfig {
    pub global: bool,
    pub channels: HashSet<ChannelId>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, poise::ChoiceParameter)]
pub enum SafewordLocation {
    Global,
    Here
}

impl SafewordConfig {
    pub fn add_safeword(&mut self, location: SafewordLocation, channel: ChannelId) -> bool {
        match location {
            SafewordLocation::Global => {let ret = !self.global; self.global = true; ret},
            SafewordLocation::Here => self.channels.insert(channel)
        }
    }

    pub fn remove_safeword(&mut self, location: SafewordLocation, channel: ChannelId) -> bool {
        match location {
            SafewordLocation::Global => {let ret = self.global; self.global=false; ret},
            SafewordLocation::Here => self.channels.remove(&channel)
        }
    }

    pub fn is_safewording(&self, channel: ChannelId) -> bool {
        self.global || self.channels.contains(&channel)
    }
}
