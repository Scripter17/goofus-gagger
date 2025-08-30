//! Allows setting the default values of gags.

use poise::structs::Context;
use serenity::model::{user::{User}, guild::Member};

use crate::types::*;

/// Set the default values for gags.
#[poise::command(slash_command, subcommands("global", "server", "user", "member"))]
pub async fn gag_default(
    _: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    unreachable!()
}

/// Set the global default values for gags.
#[poise::command(slash_command)]
pub async fn global(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The gag to use"]
    mode: Option<GagModeName>,
    #[description = "If true, stops you from ungagging yourself"]
    tie: Option<bool>
) -> Result<(), serenity::Error> {
    let new_diff = GagConfigDiff {mode, tie};

    ctx.data().gag_defaults.write().expect("No panics").entry(ctx.author().id).or_default().global = new_diff;

    ctx.say(format!("Globally set your gag defaults to `{}`", serde_json::to_string(&new_diff).expect("Serialization to never fail"))).await?;

    Ok(())
}

/// Set the per-server default values for gags
#[poise::command(slash_command, guild_only)]
pub async fn server(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The gag to use"]
    mode: Option<GagModeName>,
    #[description = "If true, stop the gaggee from ungagging themself"]
    tie: Option<bool>
) -> Result<(), serenity::Error> {
    let new_diff = GagConfigDiff {mode, tie};

    ctx.data().gag_defaults.write().expect("No panics").entry(ctx.author().id).or_default().per_guild.insert(ctx.guild_id().expect("The /gag_default server command to only be invocable in servers"), new_diff);

    ctx.say(format!("Set your gag defaults for this server to `{}`", serde_json::to_string(&new_diff).expect("Serialization to never fail"))).await?;

    Ok(())
}

/// Set the per-user default values for gags
#[poise::command(slash_command)]
pub async fn user(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The user to set the default for"]
    user: User,
    #[description = "The gag to use"]
    mode: Option<GagModeName>,
    #[description = "If true, stops you from ungagging yourself"]
    tie: Option<bool>
) -> Result<(), serenity::Error> {
    let new_diff = GagConfigDiff {mode, tie};

    ctx.data().gag_defaults.write().expect("No panics").entry(ctx.author().id).or_default().per_user.insert(user.id, new_diff);

    ctx.say(format!("Set your gag defaults for {user} to `{}`", serde_json::to_string(&new_diff).expect("Serialization to never fail"))).await?;

    Ok(())
}

/// Set the per-member default values for gags
#[poise::command(slash_command, guild_only)]
pub async fn member(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The member to set the default for"]
    member: Member,
    #[description = "The gag to use"]
    mode: Option<GagModeName>,
    #[description = "If true, stops you from ungagging yourself"]
    tie: Option<bool>
) -> Result<(), serenity::Error> {
    let new_diff = GagConfigDiff {mode, tie};

    ctx.data().gag_defaults.write().expect("No panics").entry(ctx.author().id).or_default().per_member.insert(MemberId::from_member(&member), new_diff);

    ctx.say(format!("Set your gag defaults for {member} to `{}`", serde_json::to_string(&new_diff).expect("Serialization to never fail"))).await?;

    Ok(())
}
