use poise::structs::Context;

use crate::types::*;

/// Export your data.
#[poise::command(track_edits, slash_command, dm_only)]
pub async fn export(
    ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    todo!()
}

/// Import your data
#[poise::command(track_edits, slash_command, dm_only)]
pub async fn import(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The data to import"]
    data: String
) -> Result<(), serenity::Error> {
    todo!()
}

