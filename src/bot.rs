use crate::helper::SerenityErrorHandler;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Activity, Member};
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use serenity::{async_trait, model::user::OnlineStatus};
use tracing::{error, info};

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Set Presence of Bot
        let activity = Activity::listening("hermit help");
        let status = OnlineStatus::Idle;
        ctx.set_presence(Some(activity), status).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().starts_with("hello hermit") {
            msg.channel_id.broadcast_typing(&ctx).await.handle_result();

            let response = format!("Hello <@{}>", msg.author.id);
            msg.reply(&ctx, response).await.handle_result();
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if let Some(channel) = new_member.default_channel(&ctx.cache) {
            let message = format!("{} Joined In.", new_member.mention());
            channel
                .id
                .send_message(ctx, |m| m.content(message))
                .await
                .handle_result();
        }
    }

    async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, new: VoiceState) {
        let manager = songbird::get(&ctx)
            .await
            .expect("Songbird Client Not Initialized")
            .clone();

        let guild_id = new.guild_id.unwrap();

        let channel_id = match manager.get(guild_id) {
            Some(handle) => handle.lock().await.current_channel(),
            None => None,
        };

        if let Some(channel_id) = channel_id {
            let channel = ctx.http.get_channel(channel_id.0).await.unwrap();

            let channel_members = channel.guild().unwrap().members(&ctx.cache).await.unwrap();

            let mut has_any_user = false;
            for member in channel_members {
                if !member.user.bot {
                    has_any_user = true;
                    break;
                }
            }

            if !has_any_user {
                if let Some(handle) =  manager.get(guild_id) {
                    handle.lock().await.stop(); // Stop Playing is anything
                }

                if let Err(e) = manager.remove(guild_id).await {
                    error!("Error: {:?}", e);
                }
            }
        }
    }
}
