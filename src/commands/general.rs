use serenity::framework::standard::macros::{group, command};
use serenity::framework::standard::CommandResult;
use serenity::model::Timestamp;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::Color;
use crate::helper::SerenityErrorHandler;

#[group]
#[commands(ping, details)]
struct General;

#[command]
#[description = "Reply With Pong!"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await.handle_result();
    msg.reply(ctx, "Pong!").await.handle_result();

    Ok(())
}

#[command]
#[description = "Get Server Information: User should have Admin role"]
#[allowed_roles("Admin")]
async fn details(ctx: &Context,msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await.handle_result();

    let server = msg.guild(&ctx.cache).unwrap();
    let server_name = &server.name;
    let thumbnail = &server.icon_url();
    let owner = server.owner_id.to_user(&ctx.http).await?;
    let members = server.members;
    let members_count = members.len();

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!("{} Info:", server_name))
                    .field("Owner", owner.name, false)
                    .field("Server ID", server.id.0, false)
                    .field("Member Count", members_count, false)
                    .color(Color::FABLED_PINK);
                if let Some(url) = thumbnail {
                    e.thumbnail(url)
                } else {
                    e
                }
            })
        })
        .await.handle_result();

    let mut message = String::new();
    for member in members.into_values() {
        let content = format!(
            "Member name: {}\nID: {}\nJoined at: {}\n\n",
            &member.user.name,
            &member.user.id,
            &member.joined_at.unwrap_or(Timestamp::now()),
        );

        message.push_str(&content);
    }
    msg.channel_id
        .send_message(ctx, |m| m.content(message))
        .await?;

    Ok(())
}
