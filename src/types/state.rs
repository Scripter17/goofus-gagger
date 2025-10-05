//! The overall bot state.

use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use serde::{Serialize, Deserialize};
use serenity::model::{channel::{Message, MessageReference, MessageReferenceKind}, id::{UserId, ChannelId}, timestamp::Timestamp};
use thiserror::Error;

use crate::types::*;

/// The current state of the bot.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    /// The [`GaggeeTrust`] for each user.
    #[serde(default)]
    pub trusts: RwLock<HashMap<UserId, GaggeeTrust>>,
    /// The [`Gag`]s for each user.
    #[serde(default)]
    pub gags: RwLock<HashMap<UserId, HashMap<ChannelId, Gag>>>,
    /// The length of a message to gag for each user.
    #[serde(default)]
    pub max_msg_lengths: RwLock<HashMap<UserId, usize>>,
    /// The [`Safewords`]s for each user.
    #[serde(default)]
    pub safewords: RwLock<HashMap<UserId, Safewords>>,
    /// Default values for a [`Gag`].
    #[serde(default)]
    pub gag_defaults: RwLock<HashMap<UserId, GagDefaults>>
}

/// The errors that [`State::gag`] can return.
#[derive(Debug, Error)]
pub enum GagError {
    /// Tried to gag someone without their consent.
    #[error("Tried to gag someone without their consent.")]
    NoConsentForGag,
    /// Tried to tie someone without their consent.
    #[error("Tried to tie someone without their consent.")]
    NoConsentForTie,
    /// Tried to gag someone in a mode they haven't consented to.
    #[error("Tried to gag someone in a mode they haven't consented to.")]
    NoConsentForMode,
    /// Tried to gag someone who was already gagged.
    #[error("Tried to gag someone who was already gagged.")]
    AlreadyGagged
}

/// The errors that [`State::ungag`] can return.
#[derive(Debug, Error)]
pub enum UngagError {
    /// Treid to ungag someone without their consent.
    #[error("Treid to ungag someone without their consent.")]
    NoConsentForUngag,
    /// Tried to untie someone else without their consent.
    #[error("Tried to untie someone else without their consent.")]
    NoConsentForUntie,
    /// Tried to ungag someone from a mode they haven't consented to you ungagging them from.
    #[error("Tried to ungag someone from a mode they haven't consented to you ungagging them from.")]
    NoConsentForMode(GagModeName),
    /// Tried to untie yourself.
    #[error("Tried to untie yourself")]
    CantUntieYourself,
    /// Tried to ungag someone who wasn't gagged.
    #[error("Tried to ungag someone who wasn't gagged.")]
    WasntGagged
}


#[derive(Debug, Error)]
pub enum ChangeGagError {
    /// Tried to gag someone without their consent.
    #[error("Tried to gag someone without their consent.")]
    NoConsentForGag,
    /// Tried to gag someone in a mode they haven't consented to.
    #[error("Tried to gag someone in a mode they haven't consented to.")]
    NoConsentForMode(GagModeName),
    /// Tried to ungag someone who wasn't gagged.
    #[error("Tried to ungag someone who wasn't gagged.")]
    WasntGagged
}

/// The errors [`State::tie`] can return.
#[derive(Debug, Error)]
pub enum TieError {
    /// Tried to tie someone who wasn't gagged.
    #[error("Tried to tie someone who wasn't gagged.")]
    WasntGagged,
    /// Tried to tie someone who was already tied.
    #[error("Tried to tie someone who was already tied.")]
    AlreadyTied,
    /// Tried to tie someone who doesn't consent to you tying them in any gag mode.
    #[error("Tried to tie someone who doesn't consent to you tying them in any gag mode.")]
    NoConsentForTie,
    /// Tried to tie someone who doesn't consent to you tying them in their current gag's mode.
    #[error("Tried to tie someone who doesn't consent to you tying them in their current gag's mode.")]
    NoConsentForMode(GagModeName)
}

/// The errors [`State::untie`] can return.
#[derive(Debug, Error)]
pub enum UntieError {
    /// Tried to untie someone who wasn't gagged.
    #[error("Tried to untie someone who wasn't gagged.")]
    WasntGagged,
    /// Tried to untie someone who wasn't tied.
    #[error("Tried to untie someone who wasn't tied.")]
    WasntTied,
    /// Tried to untie someone who doesn't consent to you untying them in any gag mode.
    #[error("Tried to untie someone who doesn't consent to you untying them in any gag mode.")]
    NoConsentForUntie,
    /// You can't untie yourself.
    #[error("You can't untie yourself.")]
    CantUntieYourself,
    /// Tried to untie someone who doesn't consent to you untying them in their current gag's mode.
    #[error("Tried to untie someone who doesn't consent to you untying them in their current gag's mode.")]
    NoConsentForMode(GagModeName)
}

impl State {
    /// Does cleanup stuff for both privacy and not giving weird answers about expired gags.
    ///
    /// Yes this does invalidate the entire point of both using [`RwLock`] and using multiple of them.
    pub fn cleanup(&self, now: Timestamp) {
        self.trusts.write().expect("No pancis").retain(|_, x| x != &GaggeeTrust::default());
        let mut gags_lock = self.gags.write().expect("No panics");
        for user_gags in gags_lock.values_mut() {user_gags.retain(|_, x| x.until.is_none_or(|until| now < until));}
        gags_lock.retain(|_, x| x != &HashMap::<_, _>::default());
        self.max_msg_lengths.write().expect("No panics").retain(|_, x| *x != default_max_msg_length());
        self.safewords.write().expect("No panics").retain(|_, x| x != &Safewords::default());
        self.gag_defaults.write().expect("No panics").retain(|_, x| x != &GagDefaults::default());
    }

    /// Tie a gaggee.
    pub fn tie(&self, gaggee: UserId, gagger: MemberId, new_tie: NewTie) -> Result<(), TieError> {
        let trust = self.trust_for(gaggee, gagger);

        if !trust.tie {Err(TieError::NoConsentForTie)?}

        let mut lock = self.gags.write().expect("No panics");
        let gag = lock.get_mut(&gaggee).ok_or(TieError::WasntGagged)?
            .get_mut(&new_tie.channel).ok_or(TieError::WasntGagged)?;

        if !trust.gag_modes.contains(&gag.config.mode) {Err(TieError::NoConsentForMode(gag.config.mode))?}
        if gag.config.tie {Err(TieError::AlreadyTied)?}

        gag.config.tie = true;

        Ok(())
    }

    /// Untie a gaggee.
    pub fn untie(&self, gaggee: UserId, gagger: MemberId, new_untie: NewUntie) -> Result<(), UntieError> {
        let trust = self.trust_for(gaggee, gagger);

        if !trust.untie {
            if gaggee == gagger.user {
                Err(UntieError::CantUntieYourself)?
            } else {
                Err(UntieError::NoConsentForUntie)?
            }
        }

        let mut lock = self.gags.write().expect("No panics");
        let gag = lock.get_mut(&gaggee).ok_or(UntieError::WasntGagged)?
            .get_mut(&new_untie.channel).ok_or(UntieError::WasntGagged)?;

        if !trust.gag_modes.contains(&gag.config.mode) {Err(UntieError::NoConsentForMode(gag.config.mode))?}
        if !gag.config.tie {Err(UntieError::WasntTied)?}

        gag.config.tie = false;

        Ok(())
    }

    /// Get the [`MessageAction`] to do for a [`Message`].
    pub fn get_action(&self, msg: &Message) -> Option<MessageAction> {
        if matches!(msg.message_reference, Some(MessageReference {kind: MessageReferenceKind::Forward, ..})) {return None;}
        let gags_lock = self.gags.read().expect("No panics");
        let gag = gags_lock.get(&msg.author.id)?.get(&msg.channel_id)?;
        if gag.until.is_none_or(|until| msg.timestamp <= until) && !self.safewords.read().expect("No panics").get(&msg.author.id).is_some_and(|safeword| safeword.is_safewording(msg.channel_id, msg.guild_id)) {
            let max_msg_length = self.max_msg_lengths.read().expect("No panics").get(&msg.author.id).copied().unwrap_or(default_max_msg_length());
            if msg.content.len() <= max_msg_length {
                Some(MessageAction::Gag(gag.config.mode))
            } else {
                Some(MessageAction::WarnTooLong(max_msg_length))
            }
        } else {
            None
        }
    }

    /// Gag a user.
    pub fn gag(&self, gaggee: UserId, gagger: MemberId, new_gag: NewGag) -> Result<(), GagError> {
        let trust = self.trust_for(gaggee, gagger);

        if !trust.gag {Err(GagError::NoConsentForGag)?}
        if new_gag.gag.config.tie && !trust.tie  {Err(GagError::NoConsentForTie)?}
        if !trust.gag_modes.contains(&new_gag.gag.config.mode) {Err(GagError::NoConsentForMode)?}

        match self.gags.write().expect("No panics").entry(gaggee).or_default().entry(new_gag.channel) {
            Entry::Occupied(_) => Err(GagError::AlreadyGagged)?,
            Entry::Vacant(e) => {e.insert(new_gag.into());}
        }

        Ok(())
    }

    /// Ungag a user.
    pub fn ungag(&self, gaggee: UserId, gagger: MemberId, new_ungag: NewUngag) -> Result<(), UngagError> {
        let trust = self.trust_for(gaggee, gagger);

        if !trust.ungag {Err(UngagError::NoConsentForUngag)?}

        match self.gags.write().expect("No panics").get_mut(&gaggee).ok_or(UngagError::WasntGagged)?.entry(new_ungag.channel) {
            Entry::Occupied(gag) => {
                if gag.get().config.tie && !trust.untie {
                    if gaggee == gagger.user {
                        Err(UngagError::CantUntieYourself)?
                    } else {
                        Err(UngagError::NoConsentForUntie)?
                    }
                }
                if !trust.gag_modes.contains(&gag.get().config.mode) {Err(UngagError::NoConsentForMode(gag.get().config.mode))?}

                gag.remove();
            },
            Entry::Vacant(_) => Err(UngagError::WasntGagged)?
        }

        Ok(())
    }

    /// Change a gaggee's gag to `mode`.
    ///
    /// Returns the old [`GagModeName`].
    pub fn change_gag(&self, gaggee: UserId, gagger: MemberId, change_gag: ChangeGag) -> Result<GagModeName, ChangeGagError> {
        let trust = self.trust_for(gaggee, gagger);

        if !trust.gag {Err(ChangeGagError::NoConsentForGag)?;}
        if !trust.gag_modes.contains(&change_gag.mode) {Err(ChangeGagError::NoConsentForMode(change_gag.mode))?;}

        let old = match self.gags.write().expect("No panics").entry(gaggee).or_default().get_mut(&change_gag.channel) {
            Some(gag) => {let old = gag.config.mode; gag.config.mode = change_gag.mode; old},
            None => Err(ChangeGagError::WasntGagged)?
        };

        Ok(old)
    }

    /// Get a user's [`Trust`] for a member.
    pub fn trust_for(&self, gaggee: UserId, gagger: MemberId) -> Trust {
        if gaggee == gagger.user {
            Trust::for_self()
        } else if let Some(gaggee_trust) = self.trusts.read().expect("No panics").get(&gaggee) {
            let mut ret = gaggee_trust.global.clone();
            if let Some(diff) = gaggee_trust.per_guild .get(&gagger.guild) {diff.apply(&mut ret);}
            if let Some(diff) = gaggee_trust.per_user  .get(&gagger.user ) {diff.apply(&mut ret);}
            if let Some(diff) = gaggee_trust.per_member.get(&gagger      ) {diff.apply(&mut ret);}
            ret
        } else {
            Default::default()
        }
    }

    /// Export a user's data.
    pub fn export(&self, user: UserId) -> PortableGaggee {
        PortableGaggee {
            trusts        : self.trusts         .read().expect("No panics").get(&user).cloned(),
            gags          : self.gags           .read().expect("No panics").get(&user).cloned(),
            max_msg_length: self.max_msg_lengths.read().expect("No panics").get(&user).cloned(),
            safewords     : self.safewords      .read().expect("No panics").get(&user).cloned(),
            gag_defaults  : self.gag_defaults   .read().expect("No panics").get(&user).cloned()
        }
    }

    /// Import a user's data.
    pub fn import(&self, user: UserId, data: PortableGaggee) {
        let PortableGaggee {trusts, gags, max_msg_length, safewords, gag_defaults} = data;
        
        match trusts {
            Some(trusts) => {self.trusts.write().expect("No panics").insert(user, trusts);},
            None         => {self.trusts.write().expect("No panics").remove(&user);}
        }
        match gags {
            Some(gags) => {self.gags.write().expect("No panics").insert(user, gags);},
            None       => {self.gags.write().expect("No panics").remove(&user);}
        }
        match max_msg_length {
            Some(max_msg_length) => {self.max_msg_lengths.write().expect("No panics").insert(user, max_msg_length);},
            None                 => {self.max_msg_lengths.write().expect("No panics").remove(&user);}
        }
        match safewords {
            Some(safewords) => {self.safewords.write().expect("No panics").insert(user, safewords);},
            None            => {self.safewords.write().expect("No panics").remove(&user);}
        }
        match gag_defaults {
            Some(gag_defaults) => {self.gag_defaults.write().expect("No panics").insert(user, gag_defaults);},
            None               => {self.gag_defaults.write().expect("No panics").remove(&user);}
        }
    }
}

/// The default max length of a message to gag.
///
/// Currently 256.
pub fn default_max_msg_length() -> usize {
    256
}
