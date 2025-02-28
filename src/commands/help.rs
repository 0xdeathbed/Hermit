use crate::{Context, Error};
use poise::command;

// Help Command: Give info about all commands provided by bot
#[command(prefix_command, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx).await?;

    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "___",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
