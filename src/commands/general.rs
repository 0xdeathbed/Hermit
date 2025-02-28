use crate::services::joke;
use crate::services::meme;
use crate::{Context, Error};
use poise::command;
use poise::CreateReply;
use serenity::all::Colour;
use serenity::all::CreateEmbed;
use serenity::all::Timestamp;
use tracing::error;

/// Reply `Pong!`
#[command(prefix_command, slash_command, broadcast_typing)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;
    Ok(())
}

/// Fetch a joke From JokeApi
#[command(prefix_command, slash_command, broadcast_typing)]
pub async fn joke(ctx: Context<'_>) -> Result<(), Error> {
    match joke::joke_from_joke_api().await {
        Ok(joke_resp) => _ = ctx.say(joke_resp.joke).await?,
        Err(e) => {
            error!("{:?}", e);
        }
    }

    Ok(())
}

/// Fetch a meme From Meme-Api
#[command(prefix_command, slash_command, broadcast_typing)]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    match meme::get_meme().await {
        Ok(url) => {
            ctx.send(CreateReply::default().embed(CreateEmbed::new().image(url)))
                .await?;
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }

    Ok(())
}

// #[allowed_roles("Admin")]
/// Get Server Information: User should have Admin role
#[command(prefix_command, slash_command, broadcast_typing, guild_only)]
pub async fn details(ctx: Context<'_>) -> Result<(), Error> {
    let server = ctx.guild().unwrap().clone();
    let server_name = &server.name;
    let thumbnail = server.icon_url();
    let owner = server.owner_id;
    let owner = owner.to_user(ctx).await?;
    let members = server.members.clone();
    let members_count = members.len();

    ctx.send(CreateReply::default().embed({
        let e = CreateEmbed::new()
            .title(format!("{} Info: ", server_name))
            .fields(vec![
                ("Owner", owner.name, false),
                ("Server ID", server.id.to_string(), false),
                ("Memeber Count", members_count.to_string(), false),
            ])
            .color(Colour::FABLED_PINK);

        if let Some(url) = thumbnail {
            e.thumbnail(url)
        } else {
            e
        }
    }))
    .await?;

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
    ctx.send(CreateReply::default().content(message)).await?;

    Ok(())
}
