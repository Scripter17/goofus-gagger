use crate::types::*;

/// The config determining consents, safewords, and gags.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Per-user configs.
    pub gaggees: HashMap<UserId, Gaggee>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageAction {
    Gag,
    WarnTooLong(usize),
    Nothing
}

impl Config {
    /// Returns [`true`] if a message should be gagged.
    pub fn should_do(&self, msg: &Message) -> MessageAction {
        // self.gaggees.get(&msg.author.id).is_some_and(|gaggee| !gaggee.safewords.is_safewording(msg.channel_id, msg.guild_id) && gaggee.gags.get(&msg.channel_id).is_some_and(|gag| gag.until.is_forever_or(|until| &msg.timestamp < until)))

        match self.gaggees.get(&msg.author.id) {
            Some(gaggee) => match !gaggee.safewords.is_safewording(msg.channel_id, msg.guild_id) && gaggee.gags.get(&msg.channel_id).is_some_and(|gag| gag.until < msg.timestamp) {
                true => match msg.content.len() < gaggee.max_message_length_to_gag {
                    true => MessageAction::Gag,
                    false => MessageAction::WarnTooLong(gaggee.max_message_length_to_gag)
                },
                false => MessageAction::Nothing
            },
            None => MessageAction::Nothing
        }
    }

    /// Tries to gag `gaggee` using their trust for `member`.
    pub fn gag(&mut self, gaggee: UserId,  gagger: MemberId, channel: ChannelId, gag: Gag) -> Result<(), GagError> {
        match self.gaggees.get_mut(&gaggee) {
            Some(gaggee) => gaggee.gag(gagger, channel, gag),
            None => Err(GagError::CantGag)
        }
    }

    /// Tries to ungag `gaggee` using their trust for `member`.
    pub fn ungag(&mut self, gaggee: UserId,  gagger: MemberId, channel: ChannelId) -> Result<(), UngagError> {
        match self.gaggees.get_mut(&gaggee) {
            Some(gaggee) => gaggee.ungag(gagger, channel),
            None => Err(UngagError::WasntGagged)
        }
    }
}
