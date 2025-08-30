//! Tying and untying gaggees without having un ungag and regag them.

use poise::structs::Context;
use serenity::model::user::User;

use crate::types::*;

/// Tie a gaggee
#[poise::command(slash_command, guild_only)]
pub async fn tie(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The gaggee tie"]
    target: Option<User>
) -> Result<(), serenity::Error> {
    let target = target.as_ref().unwrap_or(ctx.author());

    let message = match ctx.data().tie(target.id, MemberId::from_invoker(&ctx).expect("The /tie command to only be invocable in servers."), NewTie {channel: ctx.channel_id()}) {
        Ok(())                                => format!("Tied {target}"),
        Err(TieError::WasntGagged)            => format!("{target} wasn't gagged"),
        Err(TieError::AlreadyTied)            => format!("{target} was already tied"),
        Err(TieError::NoConsentForTie)        => format!("{target} doesn't consent to you tying them"),
        Err(TieError::NoConsentForMode(mode)) => format!("{target} doesn't consent to you tying them in gag mode `{mode}`")
    };

    ctx.say(message).await?;

    Ok(())
}

/// Untie a gaggee, leaving the gag
#[poise::command(slash_command, guild_only)]
pub async fn untie(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The gaggee to untie"]
    target: Option<User>
) -> Result<(), serenity::Error> {
    let target = target.as_ref().unwrap_or(ctx.author());

    let message = match ctx.data().untie(target.id, MemberId::from_invoker(&ctx).expect("The /tie command to only be invocable in servers."), NewUntie {channel: ctx.channel_id()}) {
        Ok(())                                  => format!("Untied {target}"),
        Err(UntieError::WasntGagged)            => format!("{target} wasn't gagged"),
        Err(UntieError::WasntTied)              => format!("{target} wasn't tied"),
        Err(UntieError::NoConsentForUntie)      => format!("{target} doesn't consent to you untying them"),
        Err(UntieError::CantUntieYourself)      =>         "You can't untie yourself".to_string(),
        Err(UntieError::NoConsentForMode(mode)) => format!("{target} doesn't consent to you untying them in gag mode `{mode}`"),
    };

    ctx.say(message).await?;

    Ok(())
}
