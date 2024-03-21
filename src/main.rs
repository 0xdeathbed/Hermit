mod bot;
mod commands;
mod helper;
mod services;

use std::net::SocketAddr;

use anyhow::anyhow;
use bot::Bot;
use serenity::prelude::*;
use serenity::{async_trait, framework::StandardFramework};
use shuttle_runtime::{CustomError, SecretStore, Secrets, Service};
use songbird::SerenityInit;

use commands::{general::GENERAL_GROUP, help::MY_HELP, music::MUSIC_GROUP};

#[shuttle_runtime::main]
async fn serenity(
    #[Secrets] secret_store: SecretStore,
) -> Result<MyService, shuttle_runtime::Error> {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_PRESENCES;

    let framework = StandardFramework::new()
        .help(&MY_HELP)
        .configure(|c| {
            c.with_whitespace(true)
                .prefixes(vec!["Hermit ", "hermit ", "Ht ", "ht "])
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&MUSIC_GROUP);

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    // Ok(client.into())
    Ok(MyService(client))
}

struct MyService(pub Client);

#[async_trait]
impl Service for MyService {
    async fn bind(mut self, _adrr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        self.0.start_autosharded().await.map_err(CustomError::new)?;

        Ok(())
    }
}
