use crate::helper::SerenityErrorHandler;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::prelude::Mutex;
use songbird::{Call, Songbird};
use std::sync::Arc;
use tracing::error;

#[group]
#[commands(join, leave, play)]
struct Music;

#[command]
#[description = "Join User Voice Channel"]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();

    let channel = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let channel_id = match channel {
        Some(channel_id) => channel_id,
        None => {
            msg.reply(ctx, "Not in a voice channel")
                .await
                .handle_result();
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Could not Initialize Songbird Voice Client");

    let _handler = manager.join(guild.id, channel_id).await;

    msg.channel_id.say(ctx, "Joined Voice Channel").await.handle_result();

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Could not Initialize Songbird Voice Client");

    if let Some(_) = manager.check_voice_channel(&ctx, &msg).await {
        match manager.remove(guild.id).await {
            Ok(_) => msg
                .channel_id
                .say(ctx, "Left the Voice Channel")
                .await
                .handle_result(),
            Err(e) => {
                error!("Failed to Remove Voice Client: {e:#?}");
                msg.channel_id
                    .say(ctx, format!("Failed to Leave Voice Channel\n{:#?}", e))
                    .await
                    .handle_result();
            }
        }
    }

    Ok(())
}

#[command]
#[description = "Play Audio using Video or Audio URL"]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.reply(
                ctx,
                "Please provide keyword or URL to video/audio with command",
            )
            .await
            .handle_result();

            typing.stop();
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Could not Initialize Songbird Voice Client");
    
    if let Some(handler_lock) = manager.check_voice_channel(&ctx, &msg).await {
        let mut handler =  handler_lock.lock().await;
        let source = match songbird::input::ytdl_search(&url).await {
            Ok(source) => source,
            Err(e) => {
                error!("Songbird Error: {:#?}",e);
                msg.channel_id.say(ctx, "Error Sourcing FFMPEG").await.handle_result();
                return Ok(());
            }
        };

        let title = source.metadata.title.clone().unwrap_or("N/A".to_string());
        
        typing.stop();
        handler.play_source(source);

        msg.channel_id.say(&ctx.http, format!("Playing {title} song")).await.handle_result();
    }

    Ok(())
}

#[async_trait]
trait ChannelPresence {
    async fn check_voice_channel(&self, ctx: &Context, msg: &Message) -> Option<Arc<Mutex<Call>>>;
}

#[async_trait]
impl ChannelPresence for Arc<Songbird> {
    async fn check_voice_channel(&self, ctx: &Context, msg: &Message) -> Option<Arc<Mutex<Call>>> {
        let guild = msg.guild(ctx).unwrap();
        match self.get(guild.id) {
            Some(a) => Some(a),
            None => {
                msg.reply(ctx, "Not in a Voice Channel")
                    .await
                    .handle_result();
                None
            }
        }
    }
}
