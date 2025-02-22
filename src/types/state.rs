use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use serde::{Serialize, Deserialize};
use serenity::model::{channel::Message, id::{UserId, ChannelId}};

use crate::types::*;

/// The current state of the bot.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub trusts: RwLock<HashMap<UserId, GaggeeTrust>>,
    pub gags: RwLock<HashMap<UserId, HashMap<ChannelId, Gag>>>,
    pub max_msg_lengths: RwLock<HashMap<UserId, usize>>,
    pub safewords: RwLock<HashMap<UserId, Safewords>>
}

#[derive(Debug)]
pub enum GagError {
    NoConsentForGag,
    NoConsentForTie,
    NoConsentForMode(GagModeName),
    AlreadyGagged
}

#[derive(Debug)]
pub enum UngagError {
    NoConsentForUngag,
    NoConsentForUntie,
    NoConsentForMode(GagModeName),
    CantUntieYourself,
    WasntGagged
}

impl State {
    pub fn get_action(&self, msg: &Message) -> Option<MessageAction> {
        let gags_lock = self.gags.read().expect("No panics");
        let gag = gags_lock.get(&msg.author.id)?.get(&msg.channel_id)?;
        if gag.until.is_none_or(|until| msg.timestamp <= until) && self.safewords.read().expect("No panics").get(&msg.author.id).is_none_or(|safeword| safeword.is_safewording(msg.channel_id, msg.guild_id)) {
            let max_msg_length = self.max_msg_lengths.read().expect("No panics").get(&msg.author.id).copied().unwrap_or(default_max_msg_length());
            if msg.content.len() <= max_msg_length {
                Some(MessageAction::Gag(gag.mode))
            } else {
                Some(MessageAction::WarnTooLong(max_msg_length))
            }
        } else {
            None
        }
    }

    pub fn gag(&self, gaggee: UserId, gagger: MemberId, new_gag: NewGag) -> Result<(), GagError> {
        let trust = self.trust_for(gaggee, gagger);
        if trust.gag {
            if trust.tie >= new_gag.tie {
                if trust.gag_modes.contains(&new_gag.mode) {
                    match self.gags.write().expect("No panics").entry(gaggee).or_default().entry(new_gag.channel) {
                        Entry::Occupied(_) => Err(GagError::AlreadyGagged)?,
                        Entry::Vacant(e) => {e.insert(new_gag.into());}
                    }
                } else {
                    Err(GagError::NoConsentForMode(new_gag.mode))?
                }
            } else {
                Err(GagError::NoConsentForTie)?
            }
        } else {
            Err(GagError::NoConsentForGag)?
        }
        Ok(())
    }

    pub fn ungag(&self, gaggee: UserId, gagger: MemberId, new_ungag: NewUngag) -> Result<(), UngagError> {
        let trust = self.trust_for(gaggee, gagger);
        if trust.ungag {
            let mut lock = self.gags.write().expect("No panics");
            match lock.get_mut(&gaggee) {
                Some(gags) => match gags.entry(new_ungag.channel) {
                    Entry::Occupied(gag) => if trust.untie >= gag.get().tie {
                        if trust.gag_modes.contains(&gag.get().mode) {
                            gag.remove();
                        } else {
                            Err(UngagError::NoConsentForMode(gag.get().mode))?
                        }
                    } else if gaggee == gagger.user {
                        Err(UngagError::CantUntieYourself)?
                    } else {
                        Err(UngagError::NoConsentForUntie)?
                    },
                    Entry::Vacant(_) => Err(UngagError::WasntGagged)?
                },
                None => Err(UngagError::WasntGagged)?
            }
        } else {
            Err(UngagError::NoConsentForUngag)?
        }
        Ok(())
    }

    pub fn trust_for(&self, gaggee: UserId, gagger: MemberId) -> Trust {
        if gaggee == gagger.user {
            Trust::for_self()
        } else {
            let mut ret = Trust::default();
            if let Some(gaggee_trust) = self.trusts.read().expect("No panics").get(&gaggee) {
                if let Some(diff) = gaggee_trust.per_guild .get(&gagger.guild) {diff.apply(&mut ret);}
                if let Some(diff) = gaggee_trust.per_user  .get(&gagger.user ) {diff.apply(&mut ret);}
                if let Some(diff) = gaggee_trust.per_member.get(&gagger      ) {diff.apply(&mut ret);}
            }
            ret
        }
    }

    pub fn export(&self, user: UserId) -> PortableGaggee {
        PortableGaggee {
            trusts        : self.trusts         .read().expect("No panics").get(&user).cloned(),
            gags          : self.gags           .read().expect("No panics").get(&user).cloned(),
            max_msg_length: self.max_msg_lengths.read().expect("No panics").get(&user).cloned(),
            safewords     : self.safewords      .read().expect("No panics").get(&user).cloned()
        }
    }

    pub fn import(&self, user: UserId, data: PortableGaggee) {
        match data.trusts {
            Some(trusts) => {self.trusts.write().expect("No panics").insert(user, trusts);},
            None         => {self.trusts.write().expect("No panics").remove(&user);}
        }
        match data.gags {
            Some(gags) => {self.gags.write().expect("No panics").insert(user, gags);},
            None       => {self.gags.write().expect("No panics").remove(&user);}
        }
        match data.max_msg_length {
            Some(max_msg_length) => {self.max_msg_lengths.write().expect("No panics").insert(user, max_msg_length);},
            None                 => {self.max_msg_lengths.write().expect("No panics").remove(&user);}
        }
        match data.safewords {
            Some(safewords) => {self.safewords.write().expect("No panics").insert(user, safewords);},
            None            => {self.safewords.write().expect("No panics").remove(&user);}
        }
    }
}

pub fn default_max_msg_length() -> usize {
    256
}
