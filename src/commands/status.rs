//! Getting the state of a user in a channel.

use poise::structs::Context;
use serenity::model::user::User;

use crate::types::*;

#[poise::command(slash_command, guild_only)]
pub async fn status(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Choose the user to get the current gag, tie, and safeword status of. Omit to get your own."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    target: Option<User>
) -> Result<(), serenity::Error> {
    let target = target.as_ref().unwrap_or(ctx.author());

    let message = {
        let lock = ctx.data().gags.read().expect("No panics");
        let gag = lock.get(&target.id)
            .and_then(|gags| gags.get(&ctx.channel_id()))
            .filter(|gag| gag.until.is_none_or(|until| ctx.created_at() <= until));

        match gag {
            Some(gag) => format!("{target} has the following gag applied in this channel: `{}`", serde_json::to_string(gag).expect("Serialization to never fail")),
            None => format!("{target} doesn't have a gag applied in this channel")
        }
    };

    ctx.say(message).await?;

    Ok(())
}
