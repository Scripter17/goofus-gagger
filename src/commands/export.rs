use poise::structs::Context;

use crate::types::*;

/// Export your data.
#[poise::command(track_edits, slash_command, dm_only)]
pub async fn export(
    ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    let message = match ctx.data().config.read().expect("No panics").gaggees.get(&ctx.author().id) {
        Some(data) => format!("```Json\n{}\n```", serde_json::to_string(data).expect("Deserializing the Gaggee to never fail")),
        None => "You don't have any data".to_string()
    };

    ctx.say(message).await?;

    Ok(())
}

/// Import your data
#[poise::command(track_edits, slash_command, dm_only)]
pub async fn import(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The data to import"]
    data: String
) -> Result<(), serenity::Error> {
    let data: Gaggee = serde_json::from_str(&data).map_err(|_| serenity::Error::Other("Invalid data."))?;

    let message = if data.id == ctx.author().id {
        ctx.data().config.write().expect("No panics").gaggees.insert(ctx.author().id, data);
        "Data imported".to_string()
    } else {
        "Good try, but that's not your data.".to_string()
    };

    ctx.say(message).await?;

    Ok(())
}

