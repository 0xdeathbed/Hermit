mod bot;
mod commands;
mod helper;
mod services;

use anyhow::anyhow;
use bot::Bot;
use commands::general::*;
use commands::help::help;
use commands::music::*;
use poise::{FrameworkOptions, PrefixFrameworkOptions};
use reqwest::Client as HttpClient;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::{env, sync::Arc, time::Duration};
use tracing::{error, info};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

pub struct Data {} // Custom data passed to all

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // custom error handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx, .. } => {
            tracing::error!("Error in command `{}`: {}", ctx.command().name, error)
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                tracing::error!("Error: {e}")
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv().ok();

    // Get the discord token set in `.env`
    let token = if let Some((_, v)) = env::vars().find(|(k, _)| k == "DISCORD_TOKEN") {
        v
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_PRESENCES;

    let options = FrameworkOptions {
        commands: vec![
            ping(),
            joke(),
            meme(),
            details(),
            help(),
            join(),
            play(),
            mute(),
            unmute(),
            leave(),
        ],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("ht".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3000),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hermit"),
                poise::Prefix::Literal("Ht"),
                poise::Prefix::Literal("bot"),
            ],
            ..Default::default()
        },
        // global error handler
        on_error: |err| Box::pin(on_error(err)),
        pre_command: |ctx| {
            Box::pin(async move {
                tracing::info!("Executing command: {}", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                tracing::info!("Executing command: {}", ctx.command().qualified_name);
            })
        },
        command_check: None,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                tracing::info!("Event: {:?}", event.snake_case_name());
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                tracing::info!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Error creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| error!("Client ended: {:?}", why));
    });

    let _signal_err = tokio::signal::ctrl_c().await;
    info!("Received Ctrl-C, shutting down.");

    Ok(())
}
