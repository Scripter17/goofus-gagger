//! Change a gaggee's gag

use poise::structs::Context;
use serenity::model::user::User;

use crate::types::*;

/// Change a gaggee's gag
#[poise::command(slash_command, guild_only)]
pub async fn change_gag(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The gaggee to change the gag for"]
    target: Option<User>,
    #[description = "The mode to change to"]
    mode: GagModeName
) -> Result<(), serenity::Error> {
    let target = target.as_ref().unwrap_or(ctx.author());

    let change = ChangeGag {
        channel: ctx.channel_id(),
        mode
    };

    let message = match ctx.data().change_gag(target.id, MemberId::from_invoker(&ctx).expect("The /change_gag command to only be invocable in servers."), change) {
        Ok(old)                                     => format!("Changed {target}'s gag from {old} ({}) to {mode} ({})", old.icon(), mode.icon()),
        Err(ChangeGagError::NoConsentForGag)        => format!("{target} hasn't consented to you gagging them"),
        Err(ChangeGagError::NoConsentForMode(mode)) => format!("{target} has consented to you gagging them but not with mode {mode} ({})", mode.icon()),
        Err(ChangeGagError::WasntGagged)            => format!("{target} wasn't gagged"),
    };

    ctx.say(message).await?;

    Ok(())
}
