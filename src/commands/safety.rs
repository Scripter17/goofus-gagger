//! Prevents unwanted behavior.

use poise::structs::Context;

use crate::types::*;

/// Set the max length of a message to gag
#[poise::command(slash_command)]
pub async fn set_max_message_length_to_gag(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The max legnth of a message to gag"]
    length: usize
) -> Result<(), serenity::Error> {
    *ctx.data().max_msg_lengths.write().expect("No panics").entry(ctx.author().id).or_insert(default_max_msg_length()) = length;

    ctx.say(format!("Set your message length limit to {length}")).await?;

    Ok(())
}
