use std::fs::read_to_string;
use std::sync::RwLock;
use std::path::PathBuf;

use serenity::prelude::*;
use serenity::client::FullEvent;
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
    pub config: PathBuf,
    #[arg(long)]
    pub just_test_config: bool
}

/// Gag a message if its user has a gag and no safeword active.
fn gag_handler<'a>(ctx: &'a Context, event: &'a FullEvent, _: poise::FrameworkContext<'a, State, serenity::Error>, state: &'a State) -> BoxFuture<'a, Result<(), serenity::Error>> {
    Box::pin(async move {
        if let FullEvent::Message{new_message: msg} = event {
            let x = state.config.read().expect("No panics").should_do(msg);
            match x {
                MessageAction::Gag => {
                    msg.channel_id.send_message(
                        &ctx.http,
                        CreateMessage::new().allowed_mentions(Default::default()).content(format!("{}: {}",
                            msg.author,
                            crate::gag::gag(&msg.content)
                        ))
                    ).await?;
                    msg.delete(&ctx.http).await?;
                },
                MessageAction::WarnTooLong(max_length) => {
                    msg.reply(
                        &ctx.http,
                        format!(
                            "While you  has a gag active here, this message is {} bytes long while the maximum message length to gag is {} bytes\nYou can use `/set_max_message_length_to_gag` to increase this",
                            msg.content.len(),
                            max_length
                        )
                    ).await?;
                },
                MessageAction::Nothing => {}
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

    if args.just_test_config {return;}

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::gag(), commands::ungag(),
                commands::trust(),
                commands::safeword(), commands::unsafeword(),
                commands::export(), commands::import(),
                commands::status(),
                commands::set_max_message_length_to_gag()
            ],
            event_handler: gag_handler,
            post_command: |ctx| Box::pin(async move {ctx.data().write_to_file().expect("Writing the Config to a file to work")}),
            allowed_mentions: Some(Default::default()),
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
