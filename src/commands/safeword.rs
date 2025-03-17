//! Enabling and disabling parts of the [`Safeword`] system.

use poise::structs::Context;

use crate::types::*;

/// Activate a safeword to temporarily ungag yourself globally, per-server, and per-channel
///
/// If the global safeword is on, the server is in the per-server safeword list, and/or the channel is in the per-channel safeword list, the muffling is disabled until all relevant safewords are `/unsafeword`ed
///
/// Gags with a time limit won't have their time limit extended
#[poise::command(slash_command)]
pub async fn safeword(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Where to apply the safeword for"]
    r#where: SafewordLocation
) -> Result<(), serenity::Error> {
    let (safeword_result, relevant_safewords) = {
        let mut safeword_lock = ctx.data().safewords.write().expect("No panics");
        let safewords = safeword_lock.entry(ctx.author().id).or_default();
        let safeword_result = safewords.add_safeword(r#where, ctx.channel_id(), ctx.guild_id());
        let relevant_safewords = safewords.get_relevant_safewords(ctx.channel_id(), ctx.guild_id());

        (safeword_result, relevant_safewords)
    };

    let mut message = match (r#where, safeword_result) {
        (SafewordLocation::Global , Ok (true)                      ) => "Enabled the global safeword",
        (SafewordLocation::Global , Ok (false)                     ) => "The global safeword was already enabled",
        (SafewordLocation::Global , Err(SafewordError::NotInServer)) => unreachable!(),
        (SafewordLocation::Server , Ok (true)                      ) => "Enabled the safeword for this server",
        (SafewordLocation::Server , Ok (false)                     ) => "The safeword for this server was already enabled",
        (SafewordLocation::Server , Err(SafewordError::NotInServer)) => "You can't enable the per-server safeword when not in a server",
        (SafewordLocation::Channel, Ok (true)                      ) => "Enabled the safeword for this channel",
        (SafewordLocation::Channel, Ok (false)                     ) => "The safeword for this channel was already enabled",
        (SafewordLocation::Channel, Err(SafewordError::NotInServer)) => unreachable!()
    }.to_string();
    message.push_str(&format!("\nYou now have the following relevant safewords enabled: {relevant_safewords:?}"));

    ctx.say(message).await?;

    Ok(())
}

/// Deactivates a safeword to re-apply all gags that haven't yet expired
#[poise::command(slash_command)]
pub async fn unsafeword(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Where to revoke the safeword for"]
    r#where: SafewordLocation
) -> Result<(), serenity::Error> {
    let (unsafeword_result, relevant_safewords) = {
        let mut safeword_lock = ctx.data().safewords.write().expect("No panics");
        let safewords = safeword_lock.entry(ctx.author().id).or_default();
        let unsafeword_result = safewords.remove_safeword(r#where, ctx.channel_id(), ctx.guild_id());
        let relevant_safewords = safewords.get_relevant_safewords(ctx.channel_id(), ctx.guild_id());

        (unsafeword_result, relevant_safewords)
    };

    let mut message = match (r#where, unsafeword_result) {
        (SafewordLocation::Global , Ok (true)                      ) => "Disabled the global safeword",
        (SafewordLocation::Global , Ok (false)                     ) => "The global safeword was already disabled",
        (SafewordLocation::Global , Err(SafewordError::NotInServer)) => unreachable!(),
        (SafewordLocation::Server , Ok (true)                      ) => "Disabled the safeword for this server",
        (SafewordLocation::Server , Ok (false)                     ) => "The safeword for this server was already disabled",
        (SafewordLocation::Server , Err(SafewordError::NotInServer)) => "You can't disable the per-server safeword when not in a server",
        (SafewordLocation::Channel, Ok (true)                      ) => "Disabled the safeword for this channel",
        (SafewordLocation::Channel, Ok (false)                     ) => "The safeword for this channel was already disabled",
        (SafewordLocation::Channel, Err(SafewordError::NotInServer)) => unreachable!()
    }.to_string();

    if !relevant_safewords.is_empty() {
        message.push_str(&format!("\nYou still have the following relevant safewords active: {relevant_safewords:?}"));
    }

    ctx.say(message).await?;

    Ok(())
}
