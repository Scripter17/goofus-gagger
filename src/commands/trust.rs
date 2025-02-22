use poise::structs::Context;
use serenity::model::{guild::Member, user::User};

use crate::types::*;

/// Sets the trust levels for the current server, a user, or a member.
#[poise::command(track_edits, slash_command, subcommands("server", "user", "member"))]
pub async fn trust(
    _ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    unreachable!()
}

/// Sets the trust levels for everyone in the current server.
#[poise::command(track_edits, slash_command, guild_only)]
pub async fn server(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Overwrite the entire server's ability to gag you"]
    gag: Option<bool>,
    #[description = "Overwrite the entire server's ability to ungag you"]
    ungag: Option<bool>,
    #[description = "Overwrite the entire server's ability to tie you"]
    tie: Option<bool>,
    #[description = "Overwrite the entire server's ability to untie you"]
    untie: Option<bool>,
    #[description = "Allow the entire server to use these gag modes with you in this server"]
    allow_gag_modes: Vec<GagModeName>,
    #[description = "Disallow the entire server from use these gag modes with you in this server"]
    disallow_gag_modes: Vec<GagModeName>
) -> Result<(), serenity::Error> {
    let diff = TrustDiff {
        gag, ungag, tie, untie,
        allow_gag_modes: allow_gag_modes.into_iter().collect(),
        disallow_gag_modes: disallow_gag_modes.into_iter().collect()
    };

    let serialized = serde_json::to_string(&diff).expect("Serialization to never fail");
    let overwrote = ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().per_guild.insert(ctx.guild_id().expect("The trust server command to only be invocable in servers"), diff).is_some();

    ctx.say(match overwrote {
        true  => format!("Overwrote this server's trust with `{serialized}`"),
        false => format!("Set this server's trust to `{serialized}`")
    }).await?;

    Ok(())
}

/// Sets the trust levels for a user in any server.
#[poise::command(track_edits, slash_command)]
pub async fn user(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The member to override trusts for everywhere."]
    user: User,
    #[description = "Overwrite the user's ability to gag you in any server"]
    gag: Option<bool>,
    #[description = "Overwrite the user's ability to ungag you in any server"]
    ungag: Option<bool>,
    #[description = "Overwrite the user's ability to tie you in any server"]
    tie: Option<bool>,
    #[description = "Overwrite the user's ability to untie you in any server"]
    untie: Option<bool>,
    #[description = "Allow the user to use these gag modes with you in any server"]
    allow_gag_modes: Vec<GagModeName>,
    #[description = "Disallow the user from use these gag modes with you in any server"]
    disallow_gag_modes: Vec<GagModeName>
) -> Result<(), serenity::Error> {
    let diff = TrustDiff {
        gag, ungag, tie, untie,
        allow_gag_modes: allow_gag_modes.into_iter().collect(),
        disallow_gag_modes: disallow_gag_modes.into_iter().collect()
    };

    let serialized = serde_json::to_string(&diff).expect("Serialization to never fail");
    let overwrote = ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().per_user.insert(user.id, diff).is_some();

    ctx.say(match overwrote {
        true  => format!("Overwrote {user}'s trust with `{serialized}`"),
        false => format!("Set {user}'s trust to `{serialized}`")
    }).await?;

    Ok(())
}

/// Sets the trust levels for a user in the current server.
#[poise::command(track_edits, slash_command)]
pub async fn member(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The member to override trusts for in this server."]
    member: Member,
    #[description = "Overwrite the user's ability to gag you in this server"]
    gag: Option<bool>,
    #[description = "Overwrite the user's ability to ungag you in this server"]
    ungag: Option<bool>,
    #[description = "Overwrite the user's ability to tie you in this server"]
    tie: Option<bool>,
    #[description = "Overwrite the user's ability to untie you in this server"]
    untie: Option<bool>,
    #[description = "Allow the user to use these gag modes with you in this server"]
    allow_gag_modes: Vec<GagModeName>,
    #[description = "Disallow the user from use these gag modes with you in this server"]
    disallow_gag_modes: Vec<GagModeName>
) -> Result<(), serenity::Error> {
    let diff = TrustDiff {
        gag, ungag, tie, untie,
        allow_gag_modes: allow_gag_modes.into_iter().collect(),
        disallow_gag_modes: disallow_gag_modes.into_iter().collect()
    };

    let serialized = serde_json::to_string(&diff).expect("Serialization to never fail");
    let overwrote = ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().per_member.insert(MemberId::from_member(&member), diff).is_some();

    ctx.say(match overwrote {
        true  => format!("Overwrote {member}'s trust with `{serialized}`"),
        false => format!("Set {member}'s trust to `{serialized}`")
    }).await?;

    Ok(())
}
