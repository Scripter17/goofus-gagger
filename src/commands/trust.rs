//! Setting [`Trust`] levels.

use std::str::FromStr;
use std::collections::HashSet;

use poise::structs::Context;
use serenity::model::{guild::Member, user::User};

use crate::types::*;

/// Sets the trust levels for the current server, a user, or a member.
#[poise::command(slash_command, subcommands("server", "user", "member", "query"))]
pub async fn trust(
    _ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    unreachable!()
}

/// Sets the trust levels for everyone in the current server.
#[poise::command(slash_command, guild_only)]
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
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    allow_gag_modes: Option<String>,
    #[description = "Disallow the entire server from use these gag modes with you in this server"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    disallow_gag_modes: Option<String>
) -> Result<(), serenity::Error> {
    let diff = TrustDiff {
        gag, ungag, tie, untie,
        allow_gag_modes   : allow_gag_modes   .map(|x| x.split(',').map(FromStr::from_str).collect::<Result<HashSet<_>, _>>().expect("All gag mode names to be valid")).unwrap_or_default(),
        disallow_gag_modes: disallow_gag_modes.map(|x| x.split(',').map(FromStr::from_str).collect::<Result<HashSet<_>, _>>().expect("All gag mode names to be valid")).unwrap_or_default()
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
#[poise::command(slash_command, guild_only)]
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
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    allow_gag_modes: Option<String>,
    #[description = "Disallow the user from use these gag modes with you in any server"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    disallow_gag_modes: Option<String>
) -> Result<(), serenity::Error> {
    if ctx.author().id == user.id {
        ctx.say("You can't overwrite your trust for yourself. You can always do anything to yourself except for untying").await?;
        return Ok(());
    }

    let diff = TrustDiff {
        gag, ungag, tie, untie,
        allow_gag_modes   : allow_gag_modes   .map(|x| x.split(',').map(FromStr::from_str).collect::<Result<HashSet<_>, _>>().expect("All gag mode names to be valid")).unwrap_or_default(),
        disallow_gag_modes: disallow_gag_modes.map(|x| x.split(',').map(FromStr::from_str).collect::<Result<HashSet<_>, _>>().expect("All gag mode names to be valid")).unwrap_or_default()
    };

    let serialized = serde_json::to_string(&diff).expect("Serialization to never fail");
    let overwrote = ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().per_user.insert(user.id, diff).is_some();
    let sum = ctx.data().trust_for(ctx.author().id, MemberId {user: user.id, guild: ctx.guild_id().expect("The /trust member command to only be invokable in servers")});
    let sum_message = serde_json::to_string(&sum).expect("Serialization to never fail");
    let additional = if sum.gag_modes.is_empty() {
        match (sum.gag, sum.ungag) {
            (false, false) => "".to_string(),
            (false, true ) => format!("\nNote that while {user} technically has ungag permission, you haven't allowed them any gag modes they can ungag\nTo do so, specify the `allow_gag_modes` parameter with a comma separated list of gag modes"),
            (true , false) => format!("\nNote that while {user} technically has gag permission, you haven't allowed them any gag modes they can gag\nTo do so, specify the `allow_gag_modes` parameter with a comma separated list of gag modes"),
            (true , true ) => format!("\nNote that while {user} technically has gag and ungag permissions, you haven't allowed them any gag modes they can gag or ungag\nTo do so, specify the `allow_gag_modes` parameter with a comma separated list of gag modes")
        }
    } else {
        "".to_string()
    };

    ctx.say(match overwrote {
        true  => format!("Overwrote your global trust for {user} with `{serialized}`\nYour sum trust for {user} in this server is now `{sum_message}`{additional}"),
        false => format!("Set your global trust for {user} to `{serialized}`\nYour sum trust for {user} in this server is now `{sum_message}`{additional}")
    }).await?;

    Ok(())
}

/// Sets the trust levels for a user in the current server.
#[poise::command(slash_command, guild_only)]
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
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    allow_gag_modes: Option<String>,
    #[description = "Disallow the user from use these gag modes with you in this server"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    disallow_gag_modes: Option<String>
) -> Result<(), serenity::Error> {
    if ctx.author().id == member.user.id {
        ctx.say("You can't overwrite your trust for yourself. You can always do anything to yourself except for untying").await?;
        return Ok(());
    }

    let diff = TrustDiff {
        gag, ungag, tie, untie,
        allow_gag_modes   : allow_gag_modes   .map(|x| x.split(',').map(FromStr::from_str).collect::<Result<HashSet<_>, _>>().expect("All gag mode names to be valid")).unwrap_or_default(),
        disallow_gag_modes: disallow_gag_modes.map(|x| x.split(',').map(FromStr::from_str).collect::<Result<HashSet<_>, _>>().expect("All gag mode names to be valid")).unwrap_or_default()
    };

    let serialized = serde_json::to_string(&diff).expect("Serialization to never fail");
    let overwrote = ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().per_member.insert(MemberId::from_member(&member), diff).is_some();
    let sum = ctx.data().trust_for(ctx.author().id, MemberId::from_member(&member));
    let sum_message = serde_json::to_string(&sum).expect("Serialization to never fail");
    let additional = if sum.gag_modes.is_empty() {
        match (sum.gag, sum.ungag) {
            (false, false) => "".to_string(),
            (false, true ) => format!("\nNote that while {member} technically has ungag permission, you haven't allowed them any gag modes they can ungag\nTo do so, specify the `allow_gag_modes` parameter with a comma separated list of gag modes"),
            (true , false) => format!("\nNote that while {member} technically has gag permission, you haven't allowed them any gag modes they can gag\nTo do so, specify the `allow_gag_modes` parameter with a comma separated list of gag modes"),
            (true , true ) => format!("\nNote that while {member} technically has gag and ungag permissions, you haven't allowed them any gag modes they can gag or ungag\nTo do so, specify the `allow_gag_modes` parameter with a comma separated list of gag modes")
        }
    } else {
        "".to_string()
    };

    ctx.say(match overwrote {
        true  => format!("Overwrote your trust for {member} in this server with `{serialized}`\nYour sum trust for {member} in this server is now `{sum_message}`{additional}"),
        false => format!("Set your trust for {member} in this server to `{serialized}`\nYour sum trust for {member} in this server is now `{sum_message}`{additional}")
    }).await?;

    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn query(
    ctx: Context<'_, State, serenity::Error>,
    member: Member
) -> Result<(), serenity::Error> {
    ctx.say(format!("Your trust for {member} in this server is `{}`", serde_json::to_string(&ctx.data().trust_for(ctx.author().id, MemberId::from_member(&member))).expect("Serialization to never fail"))).await?;

    Ok(())
}
