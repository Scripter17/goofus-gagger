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
    let result = ctx.data().config.write().expect("No panics.").gaggees.entry(ctx.author().id).or_insert_with(|| Gaggee::default_for(ctx.author().id)).safewords.add_safeword(r#where.clone(), ctx.channel_id(), ctx.guild_id());

    ctx.say(match (r#where, result) {
        (SafewordLocation::Global , Ok(false))                       => "The global safeword was already enabled",
        (SafewordLocation::Global , Ok(true ))                       => "Enabled the global safeword",
        (SafewordLocation::Global , Err(SafewordError::NotInServer)) => "Can't enable a per-server safeword outside of a server",
        (SafewordLocation::Server , Ok(false))                       => "The safeword was already enabled in this server",
        (SafewordLocation::Server , Ok(true ))                       => "Enabled the safeword in this server",
        (SafewordLocation::Server , Err(SafewordError::NotInServer)) => "Can't enable a per-server safeword outside of a server",
        (SafewordLocation::Channel, Ok(false))                       => "The safeword was already enabled in this channel",
        (SafewordLocation::Channel, Ok(true ))                       => "Enabled the safeword in this channel",
        (SafewordLocation::Channel, Err(SafewordError::NotInServer)) => "Can't enable a per-server safeword outside of a server"
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
    let result = ctx.data().config.write().expect("No panics.").gaggees.entry(ctx.author().id).or_insert_with(|| Gaggee::default_for(ctx.author().id)).safewords.remove_safeword(r#where.clone(), ctx.channel_id(), ctx.guild_id());

    ctx.say(match (r#where, result) {
        (SafewordLocation::Global , Ok(false))                       => "The global safeword was already disabled",
        (SafewordLocation::Global , Ok(true ))                       => "Disabled the global safeword",
        (SafewordLocation::Global , Err(SafewordError::NotInServer)) => "Can't disable a per-server safeword outside of a server",
        (SafewordLocation::Server , Ok(false))                       => "The safeword was already disabledin this server",
        (SafewordLocation::Server , Ok(true ))                       => "Disabled the safeword in this server",
        (SafewordLocation::Server , Err(SafewordError::NotInServer)) => "Can't disable a per-server safeword outside of a server",
        (SafewordLocation::Channel, Ok(false))                       => "The safeword was already disabledin this channel",
        (SafewordLocation::Channel, Ok(true ))                       => "Disabled the safeword in this channel",
        (SafewordLocation::Channel, Err(SafewordError::NotInServer)) => "Can't disable a per-server safeword outside of a server"
    }).await?;

    Ok(())
}
