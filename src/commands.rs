use std::borrow::Cow;

use poise::structs::Context;
use poise::CreateReply;
use serenity::model::{timestamp::Timestamp, guild::Member, user::User};

use crate::types::*;

#[poise::command(track_edits, slash_command, guild_only)]
pub async fn gag(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The user to gag. Omit to gag yourself"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    target: Option<Member>,
    #[description = "Minutes to gag them for. Omit to gag forever"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    minutes: Option<u32>,
    #[description = "Optionally \"tie\" the user so they can't ungag themself"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    tie: Option<bool>
) -> Result<(), serenity::Error> {
    let target = match target {
        Some(target) => Cow::Owned(target),
        None => ctx.author_member().await.unwrap()
    };
    let gager = ctx.author_member().await.unwrap();
    let new_gag = Gag {
        until: match minutes {
            Some(x) => GagUntil::Time(Timestamp::from_unix_timestamp(ctx.created_at().unix_timestamp() + x as i64 * 60).unwrap()),
            None => GagUntil::Forever
        },
        tie: tie.unwrap_or(false)
    };

    let gag_result = ctx.data().config.write().unwrap().gag(target.user.id, &gager, ctx.channel_id(), new_gag);

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(match gag_result {
            Ok(()) => match (minutes, tie.unwrap_or(false)) {
                (None   , false) => format!("Gagged {target} here forever"),
                (None   , true ) => format!("Gagged and tied {target} here forever"),
                (Some(1), false) => format!("Gagged {target} here for 1 minute"),
                (Some(1), true ) => format!("Gagged and tied {target} here for 1 minute"),
                (Some(x), false) => format!("Gagged {target} here for {x} minutes"),
                (Some(x), true ) => format!("Gagged and tied {target} here for {x} minutes")
            },
            Err(e) => e.message(&target)
        }),
        ..Default::default()
    }).await.unwrap();

    Ok(())
}

#[poise::command(track_edits, slash_command, guild_only)]
pub async fn ungag(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The user to ungag. Omit to gag yourself."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    target: Option<Member>
) -> Result<(), serenity::Error> {
    let target = match target {
        Some(target) => Cow::Owned(target),
        None => ctx.author_member().await.unwrap()
    };
    let gager = ctx.author_member().await.unwrap();

    let ungag_result = ctx.data().config.write().unwrap().ungag(target.user.id, &gager, ctx.channel_id());

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(match ungag_result {
            Ok(()) => format!("Ungagged {target} here"),
            Err(e) => e.message(&target)
        }),
        ..Default::default()
    }).await.unwrap();

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn set_trust_for_user(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The member to override trusts for everywhere."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    user: User,
    gag: Option<bool>,
    ungag: Option<bool>,
    tie: Option<bool>,
    untie: Option<bool>
) -> Result<(), serenity::Error> {
    let trust_diff = TrustDiff {gag, ungag, tie, untie};

    let gagee_id = ctx.author().id;
    ctx.data().config.write().unwrap().gagees.entry(gagee_id).or_insert_with(|| Gagee::default_for(gagee_id)).set_trust_for_user(user.id, trust_diff);

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(format!("{} overrode {user}'s global trust to {trust_diff:?}", ctx.author())),
        ..Default::default()
    }).await.unwrap();

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn set_trust_for_member(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The member to override trusts for in this server."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: Member,
    gag: Option<bool>,
    ungag: Option<bool>,
    tie: Option<bool>,
    untie: Option<bool>
) -> Result<(), serenity::Error> {
    let trust_diff = TrustDiff {gag, ungag, tie, untie};

    let gagee_id = ctx.author().id;
    ctx.data().config.write().unwrap().gagees.entry(gagee_id).or_insert_with(|| Gagee::default_for(gagee_id)).set_trust_for_member(&member, trust_diff);

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(format!("{} overrode {member}'s trust in this server to {trust_diff:?}", ctx.author())),
        ..Default::default()
    }).await.unwrap();
    
    Ok(())
}

#[poise::command(track_edits, slash_command, guild_only)]
pub async fn set_trust_for_server(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The server to override trusts for."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    gag: Option<bool>,
    ungag: Option<bool>,
    tie: Option<bool>,
    untie: Option<bool>
) -> Result<(), serenity::Error> {
    let trust_diff = TrustDiff {gag, ungag, tie, untie};

    let gagee_id = ctx.author().id;
    ctx.data().config.write().unwrap().gagees.entry(gagee_id).or_insert_with(|| Gagee::default_for(gagee_id)).set_trust_for_guild(ctx.guild_id().unwrap(), trust_diff);

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(format!("{} overrode this server's trust to {trust_diff:?}", ctx.author())),
        ..Default::default()
    }).await.unwrap();
    
    Ok(())
}

#[poise::command(track_edits, slash_command, dm_only)]
pub async fn export(
    ctx: Context<'_, State, serenity::Error>
) -> Result<(), serenity::Error> {
    let message = match ctx.data().config.read().unwrap().gagees.get(&ctx.author().id) {
        Some(data) => format!("```Json\n{}\n```", serde_json::to_string(data).unwrap()),
        None => "You don't have any data".to_string()
    };

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(message),
        ..Default::default()
    }).await.unwrap();

    Ok(())
}

#[poise::command(track_edits, slash_command, dm_only)]
pub async fn import(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "The data to import"]
    data: String
) -> Result<(), serenity::Error> {
    let data: Gagee = serde_json::from_str(&data).map_err(|_| serenity::Error::Other("Invalid data."))?;

    let message = if data.id == ctx.author().id {
        ctx.data().config.write().unwrap().gagees.insert(ctx.author().id, data);
        "Data imported".to_string()
    } else {
        "Good try, but that's not your data.".to_string()
    };

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(message),
        ..Default::default()
    }).await.unwrap();

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn safeword(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Where to apply the safeword for."]
    r#where: SafewordLocation
) -> Result<(), serenity::Error> {
    ctx.data().config.write().unwrap().gagees.entry(ctx.author().id).or_insert_with(|| Gagee::default_for(ctx.author().id)).safeword.add_safeword(r#where.clone(), ctx.channel_id());

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(match r#where {
            SafewordLocation::Global => "Enabled the safeword here. Note that per-channel safewords are still in effect",
            SafewordLocation::Here   => "Enabled the safeword here. Note that the global safeword is still in effect"
        }.to_string()),
        ..Default::default()
    }).await.unwrap();

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn unsafeword(
    ctx: Context<'_, State, serenity::Error>,
    #[description = "Where to revoke the safeword for."]
    r#where: SafewordLocation
) -> Result<(), serenity::Error> {
    ctx.data().config.write().unwrap().gagees.entry(ctx.author().id).or_insert_with(|| Gagee::default_for(ctx.author().id)).safeword.remove_safeword(r#where.clone(), ctx.channel_id());

    ctx.send(CreateReply {
        allowed_mentions: Some(Default::default()),
        content: Some(match r#where {
            SafewordLocation::Global => "Disabled the safeword here. Note that per-channel safewords are still in effect",
            SafewordLocation::Here   => "Disabled the safeword here. Note that the global safeword is still in effect"
        }.to_string()),
        ..Default::default()
    }).await.unwrap();

    Ok(())
}
