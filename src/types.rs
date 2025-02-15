use std::collections::HashMap;
use std::sync::RwLock;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::collections::hash_map::Entry;

use serde::{Serialize, Deserialize};
use serenity::model::{channel::Message, id::{ChannelId, UserId, GuildId}, guild::Member};
use serde_with::serde_as;

use crate::util::*;

mod gag;
pub use gag::*;
mod trust;
pub use trust::*;
mod safeword;
pub use safeword::*;

/// THe current state of the bot.
pub struct State {
    /// The config to use.
    pub config: RwLock<Config>,
    /// The path the config is stored at.
    pub path: PathBuf
}

impl State {
    /// Save the config to disk.
    pub fn commit(&self) {
        let new = serde_json::to_string_pretty(&*self.config.read().unwrap()).unwrap();
        OpenOptions::new().write(true).truncate(true).open(&self.path).unwrap().write_all(new.as_bytes()).unwrap();
    }
}

/// The config determining consents, safewords, and gags.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Per-user configs.
    pub gagees: HashMap<UserId, Gagee>
}

impl Config {
    /// Returns [`true`] if a message should be gagged.
    pub fn should_gag(&self, msg: &Message) -> bool {
        self.gagees.get(&msg.author.id).is_some_and(|gagee| !gagee.safeword.is_safewording(msg.channel_id, msg.guild_id) && gagee.gags.get(&msg.channel_id).is_some_and(|gag| gag.until.is_forever_or(|until| &msg.timestamp < until)))
    }

    /// Tries to gag `gagee` using their trust for `member`.
    pub fn gag(&mut self, gagee: UserId,  member: &Member, channel: ChannelId, gag: Gag) -> Result<(), GagError> {
        match self.gagees.get_mut(&gagee) {
            Some(gagee) => gagee.gag(member, channel, gag),
            None => Err(GagError::CantGag)
        }
    }

    /// Tries to ungag `gagee` using their trust for `member`.
    pub fn ungag(&mut self, gagee: UserId,  member: &Member, channel: ChannelId) -> Result<(), UngagError> {
        match self.gagees.get_mut(&gagee) {
            Some(gagee) => gagee.ungag(member, channel),
            None => Err(UngagError::WasntGagged)
        }
    }
}

/// A person who has at some point interacted with the bot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gagee {
    /// The ID of the gagee.
    pub id: UserId,
    /// Who the gagee trusts to do what.
    #[serde(default, skip_serializing_if = "is_default")]
    pub trusts: GageeTrust,
    /// The gags applied to the gagee.
    #[serde(default, skip_serializing_if = "is_default")]
    pub gags: HashMap<ChannelId, Gag>,
    /// The safewords the gagee is using.
    #[serde(default, skip_serializing_if = "is_default")]
    pub safeword: SafewordConfig
}

impl Gagee {
    /// Makes a defaulted [`Gagee`] with the specified [`UserId`].
    pub fn default_for(id: UserId) -> Self {
        Self {
            id,
            trusts: Default::default(),
            gags: Default::default(),
            safeword: Default::default()
        }
    }
}

/// The set of errors trying to gag someone can return.
pub enum GagError {
    /// The gagee was already gagged.
    AlreadyGagged,
    /// You can't gag the gagee.
    CantGag,
    /// You can't tie the gagee.
    CantTie
}

impl GagError {
    /// Formats the message using the gagee.
    pub fn message(self, gagee: &Member) -> String {
        match self {
            GagError::AlreadyGagged => format!("{gagee} was already gagged"),
            GagError::CantGag       => format!("{gagee} hasn't consented to you gagging them"),
            GagError::CantTie       => format!("{gagee} hasn't consented to you tying them")
        }
    }
}

/// The set of errors trying to ungag someone can return.
pub enum UngagError {
    /// The gagee wasn't gagged.
    WasntGagged,
    /// You can't ungag the gagee.
    CantUngag,
    /// You can't untie the gagee.
    CantUntie,
    /// You can't untie yourself.
    CantUntieYourself
}

impl UngagError {
    /// Formats the message using the gagee.
    pub fn message(self, gagee: &Member) -> String {
        match self {
            UngagError::WasntGagged       => format!("{gagee} wasn't gagged"),
            UngagError::CantUngag         => format!("{gagee} hasn't consented to you ungagging them"),
            UngagError::CantUntie         => format!("{gagee} hasn't consented to you untying them"),
            UngagError::CantUntieYourself => "You can't untie yourself (use /trust to let someone else do it)".to_string()
        }
    }
}

impl Gagee {
    /// Gets the trust level for the member.
    ///
    /// Getting your own trust level always returns `Trust { gag: true, ungag: true, tie: true, untie: false }`, even if you try to override it.`
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
                self.trusts.per_guild .get(&member.guild_id                         ).copied().unwrap_or_default(),
                self.trusts.per_user  .get(&member.user .id                         ).copied().unwrap_or_default(),
                self.trusts.per_member.get(&(member.guild_id, member.user.id).into()).copied().unwrap_or_default()
            ];
            for diff in diffs {diff.apply(&mut ret);}
            ret
        }
    }

    /// Gags [`Self`] with `member`'s trust level.
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

    /// Ungags [`Self`] with `member`'s trust level.
    pub fn ungag(&mut self, member: &Member, channel: ChannelId) -> Result<(), UngagError> {
        let trust = self.trust_for(member);
        match trust.ungag {
            true => match self.gags.entry(channel) {
                Entry::Occupied(e) => match trust.untie >= e.get().tie {
                    true => {e.remove();},
                    false => match self.id == member.user.id {
                        true => Err(UngagError::CantUntieYourself)?,
                        false => Err(UngagError::CantUntie)?
                    }
                },
                Entry::Vacant(_) => Err(UngagError::WasntGagged)?
            },
            false => Err(UngagError::CantUngag)?
        }

        Ok(())
    }

    /// Sets the trust level for a user.
    pub fn set_trust_for_user(&mut self, user: UserId, diff: TrustDiff) {
        *self.trusts.per_user.entry(user).or_default() = diff;
    }

    /// Sets the trust level for a server.
    pub fn set_trust_for_guild(&mut self, guild: GuildId, diff: TrustDiff) {
        *self.trusts.per_guild.entry(guild).or_default() = diff;
    }

    /// Sets the trust level for a member
    pub fn set_trust_for_member(&mut self, member: &Member, diff: TrustDiff) {
        *self.trusts.per_member.entry((member.guild_id, member.user.id).into()).or_default() = diff;
    }
}
