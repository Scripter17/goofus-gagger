//! Lets you ask to be let out.

use poise::structs::Context;

use crate::types::*;

/// Ask to be let out
#[poise::command(slash_command)]
pub async fn let_me_out(
    ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    ctx.say(format!("*{} wants to be let out*", ctx.author())).await?;

    Ok(())
}
