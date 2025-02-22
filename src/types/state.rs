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
    pub max_msg_lengths: RwLock<HashMap<UserId, usize>>
}

#[derive(Debug)]
pub enum GagError {
    NoConsentForGag,
    NoConsentForTie,
    AlreadyGagged
}

#[derive(Debug)]
pub enum UngagError {
    NoConsentForUngag,
    NoConsentForUntie,
    CantUntieYourself,
    WasntGagged
}

impl State {
    pub fn get_action(&self, msg: &Message) -> Option<MessageAction> {
        let gags_lock = self.gags.read().expect("No panics");
        let gag = gags_lock.get(&msg.author.id)?.get(&msg.channel_id)?;
        if gag.until.is_none_or(|until| msg.timestamp <= until) {
            let max_msg_length = self.max_msg_lengths.read().expect("No panics").get(&msg.author.id).copied().unwrap_or(default_max_msg_length());
            if msg.content.len() <= max_msg_length {
                Some(MessageAction::Rewrite(gag.rewriter))
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
                match self.gags.write().expect("No panics").entry(gaggee).or_default().entry(new_gag.channel) {
                    Entry::Occupied(_) => Err(GagError::AlreadyGagged)?,
                    Entry::Vacant(e) => {e.insert(new_gag.into());}
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
            match self.gags.write().expect("No panics").get_mut(&gaggee) {
                Some(gaggee_gags) => match gaggee_gags.entry(new_ungag.channel) {
                    Entry::Occupied(e) => match (e.get().tie, trust.untie, gaggee == gagger.user) {
                        (false, _, _) | (true, true, _) => {e.remove();},
                        (true, false, false) => Err(UngagError::NoConsentForUntie)?,
                        (true, false, true) => Err(UngagError::CantUntieYourself)?
                    },
                    Entry::Vacant(_) => Err(UngagError::WasntGagged)?
                },
                None => Err(UngagError::WasntGagged)?
            }
        } else {
            Err(UngagError::NoConsentForUntie)?
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
}

pub fn default_max_msg_length() -> usize {
    256
}

