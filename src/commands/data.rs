//! Export, import, and wipe user data.

use poise::structs::Context;

use crate::types::*;

/// Export your data
#[poise::command(slash_command, dm_only)]
pub async fn export(
    ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    ctx.say(serde_json::to_string(&ctx.data().export(ctx.author().id)).expect("Serialization to never fail")).await?;

    Ok(())
}

/// Import your data
#[poise::command(slash_command, dm_only)]
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

/// Wipe ALL your data from this bot. You should use /export first
#[poise::command(slash_command, dm_only)]
pub async fn wipe_my_data(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Set to true to actually do the wipe"]
    yes_im_sure: Option<bool>
) -> Result<(), serenity::Error> {
    match yes_im_sure {
        Some(true) => {
            ctx.data().import(ctx.author().id, Default::default());

            ctx.say("Wiped all your data from the bot. I hope you made a backup!").await?;
        },
        _ => {ctx.say("You need to set the yes_im_sure parameter to true.\nPlease note that you should run `/export` first to backup your data so you can `/import` it into another instance if you want to").await?;}
    }

    Ok(())
}
