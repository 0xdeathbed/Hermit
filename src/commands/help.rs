use serenity::model::channel::Message;
use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::prelude::UserId,
    prelude::Context,
};
use std::collections::HashSet;


// Help Command: Give info about all commands provided by bot
#[help]
pub async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {

    msg.channel_id.broadcast_typing(&ctx).await?;
    help_commands::with_embeds(ctx, msg, args, options, groups, owners).await?;
    Ok(())
}
