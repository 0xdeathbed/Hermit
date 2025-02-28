use crate::{Context, Error, HttpKey};
use poise::command;
use serenity::all::Mentionable;
use serenity::async_trait;
use serenity::prelude::Mutex;
use songbird::input::{Compose, YoutubeDl};
use songbird::{Call, Event, EventContext, EventHandler, Songbird, TrackEvent};
use std::sync::Arc;
use tracing::error;

/// Join User Voice Channel
#[command(prefix_command, slash_command, broadcast_typing, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();

    let channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let channel_id = match channel {
        Some(channel_id) => channel_id,
        None => {
            ctx.reply("Not in a voice channel").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Could not Initialize Songbird Voice Client")
        .clone();

    let handler = manager.join(guild.id, channel_id).await;

    match handler {
        Ok(handler_lock) => {
            let mut handler = handler_lock.lock().await;

            ctx.say(format!("Joined {}", channel_id.mention())).await?;

            handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
        }
        Err(e) => {
            error!("Failed to Join: {:?}", e);
        }
    }

    Ok(())
}

/// leave current voice channel
#[command(prefix_command, slash_command, broadcast_typing, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Could not Initialize Songbird Voice Client")
        .clone();

    if let Some(_) = manager.check_voice_channel(&ctx).await {
        match manager.remove(guild.id).await {
            Ok(_) => ctx.say("Left the Voice Channel").await?,
            Err(e) => {
                error!("Failed to Remove Voice Client: {e:#?}");
                ctx.say("Failed to Leave Voice Channel").await?
            }
        };
    }

    Ok(())
}

/// Play Audio using Video or Audio URL
#[command(prefix_command, slash_command, guild_only, broadcast_typing)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "YT Url of Audio or Video"] args: String,
) -> Result<(), Error> {
    let input = args;

    if input.is_empty() {
        ctx.reply("Please provide keyword or URL to video/audio with command")
            .await?;

        return Ok(());
    }

    let do_search = !input.starts_with("http");
    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Must be in the typemap")
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Could not Initialize Songbird Voice Client")
        .clone();

    if let Some(handler_lock) = manager.check_voice_channel(&ctx).await {
        let mut handler = handler_lock.lock().await;

        let mut src = if do_search {
            YoutubeDl::new_search(http_client, input)
        } else {
            YoutubeDl::new(http_client, input)
        };

        let title = src
            .aux_metadata()
            .await
            .unwrap()
            .title
            .unwrap_or("N/A".to_string());

        let _trackhandle = handler.play_input(src.clone().into());
        ctx.say(format!("Playing {title} song")).await?;
    }

    Ok(())
}

/// Mute Bot in a Voice Channel
#[command(prefix_command, slash_command, guild_only, broadcast_typing)]
pub async fn mute(ctx: Context<'_>) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Could not Initialize Songbird Voice Client")
        .clone();

    if let Some(handler) = manager.check_voice_channel(&ctx).await {
        let mut handler = handler.lock().await;

        if handler.is_mute() {
            ctx.reply("Already muted.").await?;
        } else {
            match handler.mute(true).await {
                Ok(_) => ctx.say("Now Muted.").await?,
                Err(e) => ctx.say(format!("Failed: {:?}", e)).await?,
            };
        }
    }

    Ok(())
}

/// Unmute Bot in a Voice Channel
#[command(prefix_command, slash_command, guild_only, broadcast_typing)]
pub async fn unmute(ctx: Context<'_>) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Could not Initialize Songbird Voice Client")
        .clone();

    if let Some(handler) = manager.check_voice_channel(&ctx).await {
        let mut handler = handler.lock().await;

        match handler.mute(false).await {
            Ok(_) => ctx.say("Now Unmuted.").await?,
            Err(e) => ctx.say(format!("Failed: {e:?}")).await?,
        };
    };

    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl EventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                error!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}

#[async_trait]
trait ChannelPresence {
    async fn check_voice_channel(&self, ctx: &Context) -> Option<Arc<Mutex<Call>>>;
}

#[async_trait]
impl ChannelPresence for Arc<Songbird> {
    async fn check_voice_channel(&self, ctx: &Context) -> Option<Arc<Mutex<Call>>> {
        let guild = ctx.guild().unwrap().clone();
        match self.get(guild.id) {
            Some(a) => Some(a),
            None => {
                if let Err(e) = ctx.reply("Not in a Voice Channel").await {
                    error!("Error: {}", e);
                }
                None
            }
        }
    }
}
