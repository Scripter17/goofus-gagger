use std::fs::read_to_string;
use std::sync::RwLock;
use std::path::PathBuf;

use serenity::prelude::*;
use serenity::all::*;
use poise::BoxFuture;
use serenity::builder::CreateMessage;
use clap::Parser;

mod gag;
mod commands;
mod types;
mod util;

use types::*;

#[derive(Parser)]
pub struct Args {
    #[arg(long, default_value = "config.json")]
    config: PathBuf
}

fn gag_handler<'a>(ctx: &'a Context, event: &'a FullEvent, _: poise::FrameworkContext<'a, State, serenity::Error>, state: &'a State) -> BoxFuture<'a, Result<(), serenity::Error>> {
    Box::pin(async move {
        if let FullEvent::Message{new_message: msg} = event {
            if state.config.read().unwrap().should_gag(msg) {
                msg.channel_id.send_message(
                    &ctx.http,
                    CreateMessage::new().allowed_mentions(Default::default()).content(format!("{}: {}",
                        msg.member(&ctx.http).await.unwrap(),
                        crate::gag::gag(&msg.content)
                    ))
                ).await?;
                msg.delete(&ctx.http).await?;
            }
        }
        Ok(())
    })
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let state = State {
        config: RwLock::new(serde_json::from_str(&read_to_string(&args.config).expect("The config to exist")).expect("The config to be valid")),
        path: args.config
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::gag(), commands::ungag(), commands::set_trust_for_user(), commands::set_trust_for_member(), commands::set_trust_for_server(), commands::export(), commands::import()],
            event_handler: gag_handler,
            post_command: |ctx| Box::pin(async move {ctx.data().commit()}),
            ..Default::default()
        })
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(state)
            })
        })
        .build();

    let token = std::env::var("GOOFUS_GAGGER_KEY").expect("An API key in the environment variable GOOFUS_GAGGER_KEY");
    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents).framework(framework).await.expect("Error creating client.");

    client.start().await.expect("Bot to work");
}
