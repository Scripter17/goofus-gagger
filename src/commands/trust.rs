//! Setting [`Trust`] levels.

use poise::structs::Context;
use serenity::model::{guild::Member, user::User};

use crate::types::*;
use crate::util::*;

/// Sets the trust levels for the current server, a user, or a member
#[poise::command(slash_command, subcommands("global", "server", "user", "member", "query"))]
pub async fn trust(
    _ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    unreachable!()
}

/// Set trust config for everyone in any server
#[poise::command(slash_command, guild_only)]
pub async fn global(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Trust everyone in any server to gag you"]
    gag: Option<bool>,
    #[description = "Trust everyone in any server to ungag you"]
    ungag: Option<bool>,
    #[description = "Trust everyone in any server to tie you"]
    tie: Option<bool>,
    #[description = "Trust everyone in any server to untie you"]
    untie: Option<bool>,
    #[description = "Trust everyone in any server to use these gag modes"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    gag_modes: Option<String>
) -> Result<(), serenity::Error> {
    match parse_csv_gag_modes(gag_modes.as_deref()) {
        Ok(gag_modes) => {
            let trust = Trust {
                gag  : gag  .unwrap_or_default(),
                ungag: ungag.unwrap_or_default(),
                tie  : tie  .unwrap_or_default(),
                untie: untie.unwrap_or_default(),
                gag_modes
            };

            let serialized = serde_json::to_string(&trust).expect("Serialization to never fail");
            ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().global = trust;

            ctx.say(format!("Overwrote your global trust to `{serialized}`")).await?;
        },
        Err(bad_name) => {ctx.say(format!("Error! `{bad_name}` isn't a known gag mode")).await?;}
    }

    Ok(())
}

/// Set trust config for everyone this server
#[poise::command(slash_command, guild_only)]
pub async fn server(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Trust everyone in this server to gag you"]
    gag: Option<bool>,
    #[description = "Trust everyone in this server to ungag you"]
    ungag: Option<bool>,
    #[description = "Trust everyone in this server to tie you"]
    tie: Option<bool>,
    #[description = "Trust everyone in this server to untie you"]
    untie: Option<bool>,
    #[description = "Allow everyone in this server to use these gag modes"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    allow_gag_modes: Option<String>,
    #[description = "Disallow everyone in this server from using these gag modes"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    disallow_gag_modes: Option<String>
) -> Result<(), serenity::Error> {
    match (parse_csv_gag_modes(allow_gag_modes.as_deref()), parse_csv_gag_modes(disallow_gag_modes.as_deref())) {
        (Ok(allow_gag_modes), Ok(disallow_gag_modes)) => {
            let diff = TrustDiff {
                gag, ungag, tie, untie,
                allow_gag_modes,
                disallow_gag_modes
            };

            let serialized = serde_json::to_string(&diff).expect("Serialization to never fail");
            let overwrote = ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().per_guild.insert(ctx.guild_id().expect("The trust server command to only be invocable in servers"), diff).is_some();

            ctx.say(match overwrote {
                true  => format!("Overwrote this server's trust with `{serialized}`"),
                false => format!("Set this server's trust to `{serialized}`")
            }).await?;
        },
        (Err(bad_name), _) | (Ok(_), Err(bad_name)) => {ctx.say(format!("Error! `{bad_name}` isn't a known gag mode")).await?;}
    }

    Ok(())
}

/// Set trust config for a user in any server
#[poise::command(slash_command, guild_only)]
pub async fn user(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The user to set the trust config for in any server"]
    user: User,
    #[description = "Trust them to gag you in any server"]
    gag: Option<bool>,
    #[description = "Trust them to ungag you in any server"]
    ungag: Option<bool>,
    #[description = "Trust them to tie you in any server"]
    tie: Option<bool>,
    #[description = "Trust them to untie you in any server"]
    untie: Option<bool>,
    #[description = "Allow them to use these gag modes in any server"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    allow_gag_modes: Option<String>,
    #[description = "Disallow them from using these gag modes in any server"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    disallow_gag_modes: Option<String>
) -> Result<(), serenity::Error> {
    if ctx.author().id == user.id {
        ctx.say("You can't overwrite your trust for yourself. You can always do anything to yourself except for untying").await?;
        return Ok(());
    }

    match (parse_csv_gag_modes(allow_gag_modes.as_deref()), parse_csv_gag_modes(disallow_gag_modes.as_deref())) {
        (Ok(allow_gag_modes), Ok(disallow_gag_modes)) => {
            let diff = TrustDiff {
                gag, ungag, tie, untie,
                allow_gag_modes,
                disallow_gag_modes
            };

            let serialized = serde_json::to_string(&diff).expect("Serialization to never fail");
            let overwrote = ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().per_user.insert(user.id, diff).is_some();
            let sum = ctx.data().trust_for(ctx.author().id, MemberId {user: user.id, guild: ctx.guild_id().expect("The /trust member command to only be invokable in servers")});
            let sum_message = serde_json::to_string(&sum).expect("Serialization to never fail");

            let warning = match (sum.gag_modes.len(), sum.gag || sum.ungag || sum.tie || sum.untie) {
                (0  , true ) => format!("\nWarning: Your sum trust for {user} has actions but no modes. If this is an error and you don't intend to use other trust layers to manage their gag modes, re-run this command with `allow_gag_modes` set to a comma separated list of gag modes (Example: `Gag,Dog`)"),
                (1.., false) => format!("\nWarning: Your sum trust for {user} has modes but no actions. If this is an error and you don't intend to use other trust layers to manage their actions, re-run this command with any of `gag`, `ungag`, `tie`, and/or `untie` set to `True`"),
                _ => "".to_string()
            };

            ctx.say(match overwrote {
                true  => format!("Overwrote your global trust for {user} with `{serialized}`\nYour sum trust for {user} in this server is now `{sum_message}`{warning}"),
                false => format!("Set your global trust for {user} to `{serialized}`\nYour sum trust for {user} in this server is now `{sum_message}`{warning}")
            }).await?;
        },
        (Err(bad_name), _) | (Ok(_), Err(bad_name)) => {ctx.say(format!("Error! `{bad_name}` isn't a known gag mode")).await?;}
    }

    Ok(())
}

/// Set trust config for a user in this server
#[poise::command(slash_command, guild_only)]
pub async fn member(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The user to set the trust config for in this server"]
    member: Member,
    #[description = "Trust them to gag you in this server"]
    gag: Option<bool>,
    #[description = "Trust them to ungag you in this server"]
    ungag: Option<bool>,
    #[description = "Trust them to tie you in this server"]
    tie: Option<bool>,
    #[description = "Trust them to untie you in this server"]
    untie: Option<bool>,
    #[description = "Allow them to use these gag modes in this server"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    allow_gag_modes: Option<String>,
    #[description = "Disallow them from using these gag modes in this server"]
    #[autocomplete = "crate::util::csv_gag_mode_name_autocomplete"]
    disallow_gag_modes: Option<String>
) -> Result<(), serenity::Error> {
    if ctx.author().id == member.user.id {
        ctx.say("You can't overwrite your trust for yourself. You can always do anything to yourself except for untying").await?;
        return Ok(());
    }

    match (parse_csv_gag_modes(allow_gag_modes.as_deref()), parse_csv_gag_modes(disallow_gag_modes.as_deref())) {
        (Ok(allow_gag_modes), Ok(disallow_gag_modes)) => {
            let diff = TrustDiff {
                gag, ungag, tie, untie,
                allow_gag_modes,
                disallow_gag_modes
            };

            let serialized = serde_json::to_string(&diff).expect("Serialization to never fail");
            let overwrote = ctx.data().trusts.write().expect("No panics").entry(ctx.author().id).or_default().per_member.insert(MemberId::from_member(&member), diff).is_some();
            let sum = ctx.data().trust_for(ctx.author().id, MemberId::from_member(&member));
            let sum_message = serde_json::to_string(&sum).expect("Serialization to never fail");

            let warning = match (sum.gag_modes.len(), sum.gag || sum.ungag || sum.tie || sum.untie) {
                (0  , true ) => format!("\nWarning: Your sum trust for {member} has actions but no modes. If this is an error and you don't intend to use other trust layers to manage their gag modes, re-run this command with `allow_gag_modes` set to a comma separated list of gag modes (Example: `Gag,Dog`)"),
                (1.., false) => format!("\nWarning: Your sum trust for {member} has modes but no actions. If this is an error and you don't intend to use other trust layers to manage their actions, re-run this command with any of `gag`, `ungag`, `tie`, and/or `untie` set to `True`"),
                _ => "".to_string()
            };

            ctx.say(match overwrote {
                true  => format!("Overwrote your trust for {member} in this server with `{serialized}`\nYour sum trust for {member} in this server is now `{sum_message}`{warning}"),
                false => format!("Set your trust for {member} in this server to `{serialized}`\nYour sum trust for {member} in this server is now `{sum_message}`{warning}")
            }).await?;
        },
        (Err(bad_name), _) | (Ok(_), Err(bad_name)) => {ctx.say(format!("Error! `{bad_name}` isn't a known gag mode")).await?;}
    }

    Ok(())
}

/// Get the trusts between you and a user in this server
#[poise::command(slash_command, guild_only)]
pub async fn query(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The member to get the trust details of and for"]
    member: Member
) -> Result<(), serenity::Error> {
    ctx.say(format!(
        "Your trust for {member} in this server is `{}`\n{member}'s trust for you in this server is `{}`",
        serde_json::to_string(&ctx.data().trust_for(ctx.author().id, MemberId::from_member(&member))).expect("Serialization to never fail"),
        serde_json::to_string(&ctx.data().trust_for(member.user.id , MemberId::from_invoker(&ctx).expect("The /trust query command to only be invokable in servers"))).expect("Serialization to never fail")
    )).await?;

    Ok(())
}
