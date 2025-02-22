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
    todo!()
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
    todo!()
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
    todo!()
}
