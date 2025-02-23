use poise::structs::Context;
use rand::prelude::*;

use crate::types::*;

/// Struggle against the ropes
#[poise::command(track_edits, slash_command)]
pub async fn struggle(
    ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    let responses = [
        format!("*{} struggled against the ropes*", ctx.author()),
        format!("*{} uselessly struggled against the ropes*", ctx.author())
    ];

    let response = responses.choose(&mut rand::rng()).expect("A response to exist");

    ctx.say(response).await?;

    Ok(())
}
