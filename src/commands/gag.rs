//! Applying, removing, and one-time-using [`Gag`]s.

use poise::structs::Context;
use poise::CreateReply;
use serenity::model::{timestamp::Timestamp, user::User};
use serenity::builder::CreateMessage;

use crate::types::*;

/// "Gag" a user (or yourself) so all their (or your) messages get replaced with muffles
///
/// Currently only applies on a per-channel basis
///
/// Requres the user to consent to you gagging (and, if you try to, tying) them using any of the `/trust` commands
///
/// You can always ungag yourself but you can't untie yourself without using `/safeword` or `/export` and `/import`
#[poise::command(slash_command, guild_only)]
pub async fn gag(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The user to gag. Omit to gag yourself"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    target: Option<User>,
    #[description = "Minutes to gag them for. Omit to gag forever"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    minutes: Option<u32>,
    #[description = "Optionally \"tie\" the user so they can't ungag themself"]
    tie: Option<bool>,
    mode: Option<GagModeName>
) -> Result<(), serenity::Error> {
    let target = target.as_ref().unwrap_or(ctx.author());

    let member_id = MemberId::from_invoker(&ctx).expect("The gag command to only be runnable in a guiild");

    let mut gag_config = match ctx.data().gag_defaults.read().expect("No panics").get(&target.id) {
        Some(x) => x.default_for(member_id),
        None => Default::default()
    };
    GagConfigDiff {tie, mode}.apply(&mut gag_config);

    let gag_result = ctx.data().gag(target.id, member_id, NewGag {
        channel: ctx.channel_id(),
        gag: Gag {
            #[allow(clippy::arithmetic_side_effects, reason = "I don't think it can happen.")]
            until: minutes.map(|minutes| Timestamp::from_unix_timestamp(ctx.created_at().unix_timestamp() + minutes as i64 * 60).expect("Current time + u32::MAX minutes to be a valid time")),
            config: gag_config
        }
    });

    let message = match gag_result.map(|()| (minutes, gag_config.tie)) {
        Ok((None         , false))      => format!("Gagged {target} in this channel with mode {} ({}) forever"                       , gag_config.mode, gag_config.mode.icon()),
        Ok((None         , true ))      => format!("Gagged and tied {target} in this channel with mode {} ({}) forever"              , gag_config.mode, gag_config.mode.icon()),
        Ok((Some(1)      , false))      => format!("Gagged {target} in this channel with mode {} ({}) for 1 minute"                  , gag_config.mode, gag_config.mode.icon()),
        Ok((Some(1)      , true ))      => format!("Gagged and tied {target} in this channel with mode {} ({}) for 1 minute"         , gag_config.mode, gag_config.mode.icon()),
        Ok((Some(minutes), false))      => format!("Gagged {target} in this channel with mode {} ({}) for {minutes} minutes"         , gag_config.mode, gag_config.mode.icon()),
        Ok((Some(minutes), true ))      => format!("Gagged and tied {target} in this channel with mode {} ({}) for {minutes} minutes", gag_config.mode, gag_config.mode.icon()),
        Err(GagError::NoConsentForGag)  => format!("{target} hasn't consented to you gagging them"),
        Err(GagError::NoConsentForTie)  => format!("{target} hasn't consented to you tying them"),
        Err(GagError::NoConsentForMode) => format!("{target} has consented to you gagging them but not with mode {} ({})", gag_config.mode, gag_config.mode.icon()),
        Err(GagError::AlreadyGagged)    => format!("{target} was already gagged in this channel")
    };

    ctx.say(message).await?;

    Ok(())
}

/// Ungag a user or yourself
///
/// Requires the user to consent to you ungagging (and, if they're tied, untying) them using any of the `/trust` commands
///
/// You can always ungag yourself, but you can't untie yourself without using `/safeword` or `/export` and `/import`
#[poise::command(slash_command, guild_only)]
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
        Err(UngagError::NoConsentForMode(mode)) => format!("{target} has consented to you ungagging them but not with mode {mode} ({})", mode.icon()),
        Err(UngagError::CantUntieYourself)      =>         "You can't untie yourself".to_string(),
        Err(UngagError::WasntGagged)            => format!("{target} wasn't gagged in this channel")
    };

    ctx.say(message).await?;

    Ok(())
}

/// Send a message with a gag
#[poise::command(slash_command)]
pub async fn gagged(
    ctx: Context<'_, State, serenity::Error>,
    mode: Option<GagModeName>,
    message: String
) -> Result<(), serenity::Error> {
    let mode = mode.unwrap_or_else(|| match ctx.data().gag_defaults.read().expect("No panics").get(&ctx.author().id) {
        Some(x) => x.default_for(MemberId::from_invoker(&ctx).expect("The /gagged command to only be invocable in servers")),
        None => Default::default()
    }.mode);
    
    ctx.channel_id().send_message(
        ctx.http(),
        CreateMessage::new()
            .content(crate::util::to_gagged_message(&message, mode, ctx.author()))
            .allowed_mentions(Default::default())
    ).await?;

    ctx.send(CreateReply {
        content: Some("For some reason the bot has to send you *something* or it shows an error".to_string()),
        ephemeral: Some(true),
        ..Default::default()
    }).await?;

    Ok(())
}
