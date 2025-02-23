use poise::structs::Context;
use serenity::model::{timestamp::Timestamp, user::User};

use crate::types::*;

/// "Gag" a user (or yourself) so all their (or your) messages get replaced with muffles
///
/// Currently only applies on a per-channel basis
///
/// Requres the user to consent to you gagging (and, if you try to, tying) them using any of the `/trust` commands
///
/// You can always ungag yourself but you can't untie yourself without using `/safeword` or `/export` and `/import`
#[poise::command(track_edits, slash_command, guild_only)]
pub async fn gag(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The user to gag. Omit to gag yourself"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    target: Option<User>,
    #[description = "Minutes to gag them for. Omit to gag forever"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    minutes: Option<u32>,
    #[description = "Optionally \"tie\" the user so they can't ungag themself"]
    #[flag]
    tie: bool,
    mode: Option<GagModeName>
) -> Result<(), serenity::Error> {
    let target = target.as_ref().unwrap_or(ctx.author());
    
    let gag_result = ctx.data().gag(target.id, MemberId::from_invoker(&ctx).expect("The gag command to only be runnable in a guiild"), NewGag {
        channel: ctx.channel_id(),
        until: minutes.map(|minutes| Timestamp::from_unix_timestamp(ctx.created_at().unix_timestamp() + minutes as i64 * 60).expect("Current time + u32::MAX minutes to be a valid time")),
        tie,
        mode: mode.unwrap_or_default()
    });

    let message = match gag_result.map(|()| (minutes, tie)) {
        Ok((None         , false))            => format!("Gagged {target} in this channel with mode {} forever", mode.unwrap_or_default()),
        Ok((None         , true ))            => format!("Gagged and tied {target} in this channel with mode {} forever", mode.unwrap_or_default()),
        Ok((Some(minutes), false))            => format!("Gagged {target} in this channel with mode {} for {minutes} minutes", mode.unwrap_or_default()),
        Ok((Some(minutes), true ))            => format!("Gagged and tied {target} in this channel with mode {} for {minutes} minutes", mode.unwrap_or_default()),
        Err(GagError::NoConsentForGag)        => format!("{target} hasn't consented to you gagging them"),
        Err(GagError::NoConsentForTie)        => format!("{target} hasn't consented to you tying them"),
        Err(GagError::NoConsentForMode(mode)) => format!("{target} has consented to you gagging them but not with mode {mode}"),
        Err(GagError::AlreadyGagged)          => format!("{target} was already gagged in this channel")
    };

    ctx.say(message).await?;

    Ok(())
}

/// Ungag a user or yourself
///
/// Requires the user to consent to you ungagging (and, if they're tied, untying) them using any of the `/trust` commands
///
/// You can always ungag yourself, but you can't untie yourself without using `/safeword` or `/export` and `/import`
#[poise::command(track_edits, slash_command, guild_only)]
pub async fn ungag(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The user to ungag. Omit to gag yourself"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    target: Option<User>
) -> Result<(), serenity::Error> {
    let target = target.as_ref().unwrap_or(ctx.author());

    let ungag_result = ctx.data().ungag(target.id, MemberId::from_invoker(&ctx).expect("The ungag command to only be runnable in a guild"), NewUngag {channel: ctx.channel_id()});

    let message = match ungag_result {
        Ok(()) => format!("Ungagged {target} in this channel"),
        Err(UngagError::NoConsentForUngag)      => format!("{target} hasn't consented to you ungagging them"),
        Err(UngagError::NoConsentForUntie)      => format!("{target} hasn't consented to you untying them"),
        Err(UngagError::NoConsentForMode(mode)) => format!("{target} has consented to you ungagging them but not with mode {mode}"),
        Err(UngagError::CantUntieYourself)      =>         "You can't untie yourself".to_string(),
        Err(UngagError::WasntGagged)            => format!("{target} wasn't gagged in this channel")
    };

    ctx.say(message).await?;

    Ok(())
}
