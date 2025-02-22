use poise::structs::Context;

use crate::types::*;

/// Export your data.
#[poise::command(track_edits, slash_command, dm_only)]
pub async fn export(
    ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    ctx.say(serde_json::to_string(&ctx.data().export(ctx.author().id)).expect("Serialization to never fail")).await?;

    Ok(())
}

/// Import your data
#[poise::command(track_edits, slash_command, dm_only)]
pub async fn import(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The data to import"]
    data: String
) -> Result<(), serenity::Error> {
    let import_result = match serde_json::from_str(&data) {
        Ok(data) => {ctx.data().import(ctx.author().id, data); Ok(())}
        Err(_) => Err(())
    };

    ctx.say(match import_result {
        Ok(()) => "Data imported",
        Err(()) => "Invalid data"
    }).await?;

    Ok(())
}
