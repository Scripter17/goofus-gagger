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
    todo!()
}

/// Deactivates a safeword to re-apply all gags that haven't yet expired
#[poise::command(track_edits, slash_command)]
pub async fn unsafeword(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Where to revoke the safeword for"]
    r#where: SafewordLocation
) -> Result<(), serenity::Error> {
    todo!()
}
