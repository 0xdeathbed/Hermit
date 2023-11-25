use crate::helper::SerenityErrorHandler;
use crate::services::joke;
use crate::services::meme;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::Timestamp;
use serenity::prelude::Context;
use serenity::utils::Color;
use tracing::error;

#[group]
#[commands(ping, details, joke, meme)]
struct General;

#[command]
#[description = "Reply With Pong!"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await.handle_result();
    msg.reply(ctx, "Pong!").await.handle_result();

    Ok(())
}

#[command]
#[description = "Joke From JokeApi"]
async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await.handle_result();

    match joke::joke_from_joke_api().await {
        Ok(joke_resp) => msg
            .channel_id
            .say(ctx, joke_resp.joke)
            .await
            .handle_result(),
        Err(e) => {
            error!("{:?}", e);
        }
    }

    Ok(())
}

#[command]
#[description = "Meme From Meme-Api"]
async fn meme(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await.handle_result();

    match meme::get_meme().await {
        Ok(url) => {
            msg.channel_id
                .send_message(&ctx, |m| m.embed(|e| e.image(url)))
                .await
                .handle_result();
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }

    Ok(())
}

#[command]
#[description = "Get Server Information: User should have Admin role"]
#[only_in(guilds)]
#[allowed_roles("Admin")]
async fn details(ctx: &Context, msg: &Message) -> CommandResult {
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
        .await
        .handle_result();

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
