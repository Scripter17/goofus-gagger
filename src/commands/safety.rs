use poise::structs::Context;

use crate::types::*;

/// Change the max length of message that gets gagged.
#[poise::command(track_edits, slash_command)]
pub async fn set_max_message_length_to_gag(
    ctx: Context<'_, State, serenity::Error>,
    length: usize
) -> Result<(), serenity::Error> {
    todo!()
}
