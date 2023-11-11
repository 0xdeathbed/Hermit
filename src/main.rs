mod bot;
mod commands;
mod helper;

use anyhow::anyhow;
use bot::Bot;
use serenity::framework::StandardFramework;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use songbird::SerenityInit;

use commands::general::GENERAL_GROUP;
use commands::help::MY_HELP;
use commands::music::MUSIC_GROUP;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
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
                c.with_whitespace(true).prefixes(vec!["Hermit ", "hermit ", "Ht ", "ht "])
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

    Ok(client.into())
}
