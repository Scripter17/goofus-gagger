use std::borrow::Cow;

use poise::structs::Context;
use serenity::model::id::UserId;

use crate::types::*;

#[poise::command(track_edits, slash_command, guild_only)]
pub async fn status(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Choose the user to get the current gag, tie, and safeword status of. Omit to get your own."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    user: Option<UserId>
) -> Result<(), serenity::Error> {
    todo!()
}
