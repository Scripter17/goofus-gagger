use std::fs::read_to_string;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::OnceLock;

use serenity::prelude::*;
use serenity::client::FullEvent;
use poise::BoxFuture;
use serenity::builder::CreateMessage;
use clap::{Parser, Subcommand};

mod commands;
mod types;
mod util;

use types::*;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    mode: Mode
}

#[derive(Subcommand)]
enum Mode {
    RunBot {
        #[arg(long, default_value = "state.json")]
        state: PathBuf
    },
    TestGagMode {
        #[arg(long)]
        rewriter: GagModeName,
        text: String,
        #[arg(long)]
        count: u8
    }
}

/// Gag a message if its user has a gag and no safeword active.
fn gag_handler<'a>(ctx: &'a Context, event: &'a FullEvent, _: poise::FrameworkContext<'a, State, serenity::Error>, state: &'a State) -> BoxFuture<'a, Result<(), serenity::Error>> {
    Box::pin(async move {
        if let FullEvent::Message{new_message: msg} = event {
            if let Some(action) = state.get_action(msg) {
                match action {
                    MessageAction::Gag(mode) => {
                        msg.channel_id.send_message(
                            &ctx.http,
                            CreateMessage::new().allowed_mentions(Default::default()).content(format!("{}: {}",
                                msg.author,
                                mode.get().rewrite(&msg.content).expect("The rewriter to be valid")
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
                    }
                }
            }
        }
        Ok(())
    })
}

static STATE_PATH: OnceLock<PathBuf> = OnceLock::new();

#[tokio::main]
async fn main() {
    match Args::parse() {
        Args {mode: Mode::RunBot {state: state_path}} => {
            let state: State = serde_json::from_str(&read_to_string(&state_path).expect("The state file to exist")).expect("The state to be valid");

            STATE_PATH.set(state_path).expect("The STATE_PATH static to not have already been set");

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
                    post_command: move |ctx: poise::Context<'_, State, _>| Box::pin(async move {
                        OpenOptions::new().write(true).truncate(true)
                            .open(STATE_PATH.get().expect("The STATE_PATH to have been set by now"))
                            .expect("The file to be openable")
                            .write_all(serde_json::to_string_pretty(ctx.data()).expect("The state to be serializable").as_bytes())
                            .expect("The file to be writable")
                    }),
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
        },
        Args {mode: Mode::TestGagMode {rewriter, text, count}} => for _ in 0..count {
            println!("{:?}", rewriter.get().rewrite(&text));
        }
    }
}
