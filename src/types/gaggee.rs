use serde::{Serialize, Deserialize};

use crate::types::*;

/// A person who has at some point interacted with the bot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gaggee {
    /// The ID of the gaggee.
    pub id: UserId,
    /// Who the gaggee trusts to do what.
    #[serde(default, skip_serializing_if = "is_default")]
    pub trust: GaggeeTrust,
    /// The gags applied to the gaggee.
    #[serde(default, skip_serializing_if = "is_default")]
    pub gags: HashMap<ChannelId, Gag>,
    /// The safewords the gaggee is using.
    #[serde(default, skip_serializing_if = "is_default")]
    pub safewords: Safewords
}

/// The set of errors trying to gag someone can return.
pub enum GagError {
    /// The gaggee was already gagged.
    AlreadyGagged,
    /// You can't gag the gaggee.
    CantGag,
    /// You can't tie the gaggee.
    CantTie
}

/// The set of errors trying to ungag someone can return.
pub enum UngagError {
    /// The gaggee wasn't gagged.
    WasntGagged,
    /// You can't ungag the gaggee.
    CantUngag,
    /// You can't untie the gaggee.
    CantUntie,
    /// You can't untie yourself.
    CantUntieYourself
}

impl Gaggee {
    /// Makes a defaulted [`Gaggee`] with the specified [`UserId`].
    pub fn default_for(id: UserId) -> Self {
        Self {
            id,
            trust: Default::default(),
            gags: Default::default(),
            safewords: Default::default()
        }
    }

    /// Gags [`Self`] with `gagger`'s trust level.
    pub fn gag(&mut self, gagger: MemberId, channel: ChannelId, gag: Gag) -> Result<(), GagError> {
        let trust = self.trust_for(gagger);
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

    /// Ungags [`Self`] with `gagger`'s trust level.
    pub fn ungag(&mut self, gagger: MemberId, channel: ChannelId) -> Result<(), UngagError> {
        let trust = self.trust_for(gagger);
        match trust.ungag {
            true => match self.gags.entry(channel) {
                Entry::Occupied(e) => match trust.untie >= e.get().tie {
                    true => {e.remove();},
                    false => match self.id == gagger.user {
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
        self.trust.per_user.insert(user, diff);
    }

    /// Sets the trust level for a server.
    pub fn set_trust_for_guild(&mut self, guild: GuildId, diff: TrustDiff) {
        self.trust.per_guild.insert(guild, diff);
    }

    /// Sets the trust level for a member
    pub fn set_trust_for_member(&mut self, member: MemberId, diff: TrustDiff) {
        self.trust.per_member.insert(member, diff);
    }

    /// Gets the trust level for the member.
    ///
    /// Getting your own trust level always returns `Trust { gag: true, ungag: true, tie: true, untie: false }`, even if you try to override it.`
    pub fn trust_for(&self, member: MemberId) -> Trust {
        if self.id == member.user {
            Trust::for_self()
        } else {
            let mut ret = Trust::default();
            if let Some(diff) = self.trust.per_guild .get(&member.guild) {diff.apply(&mut ret);}
            if let Some(diff) = self.trust.per_user  .get(&member.user ) {diff.apply(&mut ret);}
            if let Some(diff) = self.trust.per_member.get(&member      ) {diff.apply(&mut ret);}
            ret
        }
    }
}
