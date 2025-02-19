use std::borrow::Cow;

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
    #[autocomplete = "poise::builtins::autocomplete_command"]
    tie: Option<bool>
) -> Result<(), serenity::Error> {
    let target = match target {
        Some(target) => Cow::Owned(target),
        None => Cow::Borrowed(ctx.author())
    };
    let gagger = MemberId::from_invoker(&ctx).expect("The command to only be invokable in servers");
    let new_gag = Gag {
        until: match minutes {
            Some(x) => GagUntil::Time(Timestamp::from_unix_timestamp(ctx.created_at().unix_timestamp() + x as i64 * 60).expect("A valid time")),
            None => GagUntil::Forever
        },
        tie: tie.unwrap_or(false)
    };

    let gag_result = ctx.data().config.write().expect("No panics").gag(target.id, gagger, ctx.channel_id(), new_gag);

    ctx.say(match gag_result {
        Ok(()) => {
            let action = if tie.unwrap_or(false) {"Gagged and tied"} else {"Gagged"};
            let time = match minutes {
                None => "forever".to_string(),
                Some(1) => "for 1 minute".to_string(),
                Some(x) => format!("for {x} minutes")
            };
            let additional = if ctx.data().config.read().expect("No panics").gaggees.get(&target.id).is_some_and(|gaggee| gaggee.safewords.is_safewording(ctx.channel_id(), ctx.guild_id())) {
                "\nNote that they have a safeword active. The gag will only take effect when they disable the relevant safewords"
            } else {
                ""
            };

            format!("{action} {target} {time}{additional}")
        },
        Err(GagError::AlreadyGagged) => format!("{target} was already gagged"),
        Err(GagError::CantGag)       => format!("{target} hasn't consented to you gagging them"),
        Err(GagError::CantTie)       => format!("{target} hasn't consented to you tying them")
    }).await?;

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
    let target = match target {
        Some(target) => Cow::Owned(target),
        None => Cow::Borrowed(ctx.author())
    };
    let ungagger = MemberId::from_invoker(&ctx).expect("The command to only be invokable in servers");

    let ungag_result = ctx.data().config.write().expect("No panics").ungag(target.id, ungagger, ctx.channel_id());

    ctx.say(match ungag_result {
        Ok(()) => format!("Ungagged {target} here"),
        Err(UngagError::WasntGagged)       => format!("{target} wasn't gagged"),
        Err(UngagError::CantUngag)         => format!("{target} hasn't consented to you ungagging them"),
        Err(UngagError::CantUntie)         => format!("{target} hasn't consented to you untying them"),
        Err(UngagError::CantUntieYourself) => "You can't untie yourself (use /trust to let someone else do it)".to_string()
    }).await?;

    Ok(())
}

