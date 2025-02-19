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
    #[description = "Allow the entire server to gag you"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    gag: Option<bool>,
    #[description = "Allow the entire server to ungag you"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    ungag: Option<bool>,
    #[description = "Allow the entire server to tie you"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    tie: Option<bool>,
    #[description = "Allow the entire server to untie you"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    untie: Option<bool>
) -> Result<(), serenity::Error> {
    let trust_diff = TrustDiff {gag, ungag, tie, untie};

    let gaggee_id = ctx.author().id;
    ctx.data().config.write().expect("No panics").gaggees.entry(gaggee_id).or_insert_with(|| Gaggee::default_for(gaggee_id)).set_trust_for_guild(ctx.guild_id().expect("The command to only be invokable in servers"), trust_diff);

    ctx.say(format!("{} overrode this server's trust to {trust_diff:?}", ctx.author())).await?;

    Ok(())
}

/// Sets the trust levels for a user in any server.
#[poise::command(track_edits, slash_command)]
pub async fn user(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The member to override trusts for everywhere."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    user: User,
    #[description = "Allow the user to gag you in any server"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    gag: Option<bool>,
    #[description = "Allow the user to ungag you in any server"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    ungag: Option<bool>,
    #[description = "Allow the user to tie you in any server"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    tie: Option<bool>,
    #[description = "Allow the user to untie you in any server"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    untie: Option<bool>
) -> Result<(), serenity::Error> {
    let trust_diff = TrustDiff {gag, ungag, tie, untie};

    let gaggee_id = ctx.author().id;
    if gaggee_id == user.id {
        ctx.reply(format!("You can't override your trust for yourself. Your trust for yourself is always {:?}", Trust::for_self())).await?;
    } else {
        ctx.data().config.write().expect("No panics").gaggees.entry(gaggee_id).or_insert_with(|| Gaggee::default_for(gaggee_id)).set_trust_for_user(user.id, trust_diff);
        ctx.reply(format!("{} overrode {user}'s global trust to {trust_diff:?}", ctx.author())).await?;
    }

    Ok(())
}

/// Sets the trust levels for a user in the current server.
#[poise::command(track_edits, slash_command)]
pub async fn member(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The member to override trusts for in this server."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: Member,
    #[description = "Allow the user to gag you in this server"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    gag: Option<bool>,
    #[description = "Allow the user to ungag you in this server"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    ungag: Option<bool>,
    #[description = "Allow the user to tie you in this server"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    tie: Option<bool>,
    #[description = "Allow the user to untie you in this server"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    untie: Option<bool>
) -> Result<(), serenity::Error> {
    let trust_diff = TrustDiff {gag, ungag, tie, untie};

    let gaggee_id = ctx.author().id;
    if gaggee_id == member.user.id {
        ctx.reply(format!("You can't override your trust for yourself. Your trust for yourself is always {:?}", Trust::for_self())).await?;
    } else {
        ctx.data().config.write().expect("No panics").gaggees.entry(gaggee_id).or_insert_with(|| Gaggee::default_for(gaggee_id)).set_trust_for_member(MemberId::from_member(&member), trust_diff);
        ctx.reply(format!("{} overrode {member}'s trust in this server to {trust_diff:?}", ctx.author())).await?;
    }
    
    Ok(())
}
