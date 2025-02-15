use std::collections::HashMap;
use std::sync::RwLock;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::collections::hash_map::Entry;

use serde::{Serialize, Deserialize};
use serenity::model::{channel::Message, id::{ChannelId, UserId, GuildId}, guild::Member};
use serde_with::serde_as;

mod gag;
pub use gag::*;
mod trust;
pub use trust::*;

pub struct State {
    pub config: RwLock<Config>,
    pub path: PathBuf
}

impl State {
    pub fn commit(&self) {
        let new = serde_json::to_string_pretty(&*self.config.read().unwrap()).unwrap();
        OpenOptions::new().write(true).truncate(true).open(&self.path).unwrap().write_all(new.as_bytes()).unwrap();
    }
}

#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub gagees: HashMap<UserId, Gagee>
}

impl Config {
    pub fn should_gag(&self, msg: &Message) -> bool {
        self.gagees.get(&msg.author.id).is_some_and(|gagee| gagee.gags.get(&msg.channel_id).is_some_and(|gag| gag.until.is_forever_or(|until| &msg.timestamp < until)))
    }

    pub fn gag(&mut self, gagee: UserId,  member: &Member, channel: ChannelId, gag: Gag) -> Result<(), GagError> {
        match self.gagees.get_mut(&gagee) {
            Some(gagee) => gagee.gag(member, channel, gag),
            None => Err(GagError::CantGag)
        }
    }

    pub fn ungag(&mut self, gagee: UserId,  member: &Member, channel: ChannelId) -> Result<(), UngagError> {
        match self.gagees.get_mut(&gagee) {
            Some(gagee) => gagee.ungag(member, channel),
            None => Err(UngagError::WasntGagged)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gagee {
    pub id: UserId,
    #[serde(default)]
    pub trusts: GageeTrust,
    #[serde(default)]
    pub gags: HashMap<ChannelId, Gag>
}

impl Gagee {
    pub fn default_for(id: UserId) -> Self {
        Self {
            id,
            trusts: Default::default(),
            gags: Default::default()
        }
    }
}

pub enum GagError {
    AlreadyGagged,
    CantGag,
    CantTie
}

impl GagError {
    pub fn message(self, gagee: &Member) -> String {
        match self {
            GagError::AlreadyGagged => format!("{gagee} was already gagged"),
            GagError::CantGag       => format!("{gagee} hasn't consented to you gagging them"),
            GagError::CantTie       => format!("{gagee} hasn't consented to you tying them")
        }
    }
}

pub enum UngagError {
    WasntGagged,
    CantUngag,
    CantUntie
}

impl UngagError {
    pub fn message(self, gagee: &Member) -> String {
        match self {
            UngagError::WasntGagged => format!("{gagee} wasn't gagged"),
            UngagError::CantUngag   => format!("{gagee} hasn't consented to you ungagging them"),
            UngagError::CantUntie   => format!("{gagee} hasn't consented to you untying them")
        }
    }
}

impl Gagee {
    fn trust_for(&self, member: &Member) -> Trust {
        if self.id == member.user.id {
            Trust {
                gag: true,
                ungag: true,
                tie: true,
                untie: false
            }
        } else {
            let mut ret = self.trusts.default;
            let diffs = [
                self.trusts.per_guild.get(&member.guild_id).map(|guild_trust| guild_trust.default).unwrap_or_default(),
                self.trusts.per_user .get(&member.user .id).map(|user_trust | user_trust .default).unwrap_or_default(),
                self.trusts.per_user .get(&member.user .id).and_then(|user_trust| user_trust.per_guild.get(&member.guild_id)).copied().unwrap_or_default()
            ];
            for diff in diffs {diff.apply(&mut ret);}
            ret
        }
    }

    pub fn gag(&mut self, member: &Member, channel: ChannelId, gag: Gag) -> Result<(), GagError> {
        let trust = self.trust_for(member);
        match trust.gag {
            true => match trust.tie >= gag.tie {
                true => match self.gags.entry(channel) {
                    Entry::Occupied(_) => Err(GagError::AlreadyGagged)?,
                    Entry::Vacant(e) => {e.insert(gag);}
                },
                false => Err(GagError::CantTie)?
            },
            false => Err(GagError::CantGag)?
        }

        Ok(())
    }

    pub fn ungag(&mut self, member: &Member, channel: ChannelId) -> Result<(), UngagError> {
        let trust = self.trust_for(member);
        match trust.ungag {
            true => match self.gags.entry(channel) {
                Entry::Occupied(e) => match trust.untie >= e.get().tie {
                    true => {e.remove();},
                    false => Err(UngagError::CantUntie)?
                },
                Entry::Vacant(_) => Err(UngagError::WasntGagged)?
            },
            false => Err(UngagError::CantUngag)?
        }

        Ok(())
    }

    pub fn set_trust_for_user(&mut self, user: UserId, diff: TrustDiff) {
        self.trusts.per_user.entry(user).or_default().default = diff;
    }

    pub fn set_trust_for_guild(&mut self, guild: GuildId, diff: TrustDiff) {
        self.trusts.per_guild.entry(guild).or_default().default = diff;
    }

    pub fn set_trust_for_member(&mut self, member: &Member, diff: TrustDiff) {
        self.trusts.per_user.entry(member.user.id).or_default().per_guild.insert(member.guild_id, diff);
    }
}
