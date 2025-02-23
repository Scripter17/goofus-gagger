//! Convenience wrapper around a [`GuildId`] and a [`UserId`].

use std::str::FromStr;

use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Visitor}};
use serenity::model::{guild::Member, id::{UserId, GuildId}};

/// Convenience wrapper around a [`GuildId`] and a [`UserId`].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MemberId {
    /// The [`GuildId`].
    pub guild: GuildId,
    /// The [`UserId`].
    pub user: UserId
}

impl MemberId {
    /// Makes a [`Self`] from a [`Member`]
    pub fn from_member(member: &Member) -> Self {
        Self {
            guild: member.guild_id,
            user: member.user.id
        }
    }

    /// Makes a [`Self`] from a [`poise::Context`] if it's in a server.
    pub fn from_invoker<T, E>(ctx: &poise::Context<'_, T, E>) -> Option<Self> {
        ctx.guild_id().map(|guild_id| Self {guild: guild_id, user: ctx.author().id})
    }
}

impl Serialize for MemberId {
    /// Serializes as `{self.guild},{self.user}`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{},{}", self.guild, self.user))
    }
}

/// [`Visitor`] for deserializing [`MemberId`]s.
struct MemberIdVisitor;

impl Visitor<'_> for MemberIdVisitor {
    type Value = MemberId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expected \"guild_id,user_id\"")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        value.split_once(',')
            .map(|(g, u)| Ok(MemberId {
                guild: FromStr::from_str(g).map_err(|_| E::custom("Expected \"guild_id,user_id\""))?,
                user: FromStr::from_str(u).map_err(|_| E::custom("Expected \"guild_id,user_id\""))?
            }))
            .ok_or(E::custom("Expected \"guild_id,user_id\""))?
    }
}

impl<'de> Deserialize<'de> for MemberId {
    /// Currently only deserializes from `{self.guild},{self.user}`.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(MemberIdVisitor)
    }
}
