use poise::structs::Context;

use crate::types::*;

/// Change the max length of message that gets gagged.
#[poise::command(track_edits, slash_command)]
pub async fn set_max_message_length_to_gag(
    ctx: Context<'_, State, serenity::Error>,
    length: usize
) -> Result<(), serenity::Error> {
    ctx.data().config.write().expect("No panics.").gaggees.entry(ctx.author().id).or_insert_with(|| Gaggee::default_for(ctx.author().id)).max_message_length_to_gag = length;

    ctx.say(format!("Set the max length of message that gets gagged to {length}")).await?;

    Ok(())
}
