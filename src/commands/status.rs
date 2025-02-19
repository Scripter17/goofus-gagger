use std::borrow::Cow;

use poise::structs::Context;
use serenity::model::id::UserId;

use crate::types::*;

#[poise::command(track_edits, slash_command, guild_only)]
pub async fn status(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Choose the user to get the current gag, tie, and safeword status of. Omit to get your own."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    user: Option<UserId>
) -> Result<(), serenity::Error> {
    let message = {
        let config = ctx.data().config.read().expect("No panics");
        let gaggee = match config.gaggees.get(&user.unwrap_or(ctx.author().id)) {
            Some(x) => Cow::Borrowed(x),
            None => Cow::Owned(Gaggee::default_for(ctx.author().id))
        };

        format!("Gag: `{}`\nSafeword: `{}`\nTrust for you: `{}`",
            serde_json::to_string(&gaggee.gags.get(&ctx.channel_id()).filter(|gag| gag.until > ctx.created_at())).expect("Serializing the Gag to never fail"),
            gaggee.safewords.is_safewording(ctx.channel_id(), ctx.guild_id()),
            serde_json::to_string(&gaggee.trust_for(MemberId::from_invoker(&ctx).expect("The command to only be invokable in servers"))).expect("Serializing the Trust to never fail")
        )
    };

    ctx.say(message).await?;

    Ok(())
}
