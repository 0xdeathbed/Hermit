use crate::helper::SerenityErrorHandler;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Activity, Member};
use serenity::prelude::*;
use serenity::{async_trait, model::user::OnlineStatus};
use tracing::info;

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
            let message = format!("{} Joined In.", new_member.mention() );
            channel
                .id
                .send_message(ctx, |m| m.content(message))
                .await
                .handle_result();
        }
    }

}
