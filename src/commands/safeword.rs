use poise::structs::Context;

use crate::types::*;

/// Activate a safeword to temporarily ungag yourself globally, per-server, and per-channel
///
/// If the global safeword is on, the server is in the per-server safeword list, and/or the channel is in the per-channel safeword list, the muffling is disabled until all relevant safewords are `/unsafeword`ed
///
/// Gags with a time limit won't have their time limit extended
#[poise::command(track_edits, slash_command)]
pub async fn safeword(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Where to apply the safeword for"]
    r#where: SafewordLocation
) -> Result<(), serenity::Error> {
    let safeword_result = ctx.data().safewords.write().expect("No panics").entry(ctx.author().id).or_default().add_safeword(r#where, ctx.channel_id(), ctx.guild_id());

    ctx.say(match (r#where, safeword_result) {
        (SafewordLocation::Global , Ok (true)                      ) => "Enabled the global safeword. Note that when all relevant safewords are disabled, gags will once again apply",
        (SafewordLocation::Global , Ok (false)                     ) => "The global safeword was already enabled",
        (SafewordLocation::Global , Err(SafewordError::NotInServer)) => unreachable!(),
        (SafewordLocation::Server , Ok (true)                      ) => "Enabled the safeword for this server. Note that when all relevant safewords are disabled, gags will once again apply",
        (SafewordLocation::Server , Ok (false)                     ) => "The safeword for this server was already enabled",
        (SafewordLocation::Server , Err(SafewordError::NotInServer)) => "You can't enable the per-server safeword when not in a server",
        (SafewordLocation::Channel, Ok (true)                      ) => "Enabled the safeword for this channel. Note that when all relevant safewords are disabled, gags will oncea again apply",
        (SafewordLocation::Channel, Ok (false)                     ) => "The safeword for this channel was already enabled",
        (SafewordLocation::Channel, Err(SafewordError::NotInServer)) => unreachable!()
    }).await?;

    Ok(())
}

/// Deactivates a safeword to re-apply all gags that haven't yet expired
#[poise::command(track_edits, slash_command)]
pub async fn unsafeword(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Where to revoke the safeword for"]
    r#where: SafewordLocation
) -> Result<(), serenity::Error> {
    let unsafeword_result = match ctx.data().safewords.write().expect("No panics").get_mut(&ctx.author().id) {
        Some(safeword) => safeword.remove_safeword(r#where, ctx.channel_id(), ctx.guild_id()),
        None => Ok(false)
    };

    ctx.say(match (r#where, unsafeword_result) {
        (SafewordLocation::Global , Ok (true)                      ) => "Disabled the global safeword. Note that when all relevant safewords are disabled, gags will once again apply",
        (SafewordLocation::Global , Ok (false)                     ) => "The global safeword was already disabled",
        (SafewordLocation::Global , Err(SafewordError::NotInServer)) => unreachable!(),
        (SafewordLocation::Server , Ok (true)                      ) => "Disabled the safeword for this server. Note that when all relevant safewords are disabled, gags will once again apply",
        (SafewordLocation::Server , Ok (false)                     ) => "The safeword for this server was already disabled",
        (SafewordLocation::Server , Err(SafewordError::NotInServer)) => "You can't disable the per-server safeword when not in a server",
        (SafewordLocation::Channel, Ok (true)                      ) => "Disabled the safeword for this channel. Note that when all relevant safewords are disabled, gags will oncea again apply",
        (SafewordLocation::Channel, Ok (false)                     ) => "The safeword for this channel was already disabled",
        (SafewordLocation::Channel, Err(SafewordError::NotInServer)) => unreachable!()
    }).await?;

    Ok(())
}
