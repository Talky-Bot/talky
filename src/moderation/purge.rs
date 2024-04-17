use poise::serenity_prelude::{self as serenity, Builder, GetMessages};

use crate::{error::BotError, Context};

#[poise::command(
    slash_command,
    guild_only,
    subcommands("after"),
    default_member_permissions = "MANAGE_MESSAGES"
)]
pub async fn purge(_ctx: Context<'_>, _amount: u64) -> Result<(), BotError> {
    Ok(())
}

/// Deletes all messages after the given message. Defaults to 100 messages deleted.
#[poise::command(slash_command, ephemeral)]
async fn after(
    ctx: Context<'_>,
    #[description = "The ID of the message to delete after"]
    message_id: serenity::MessageId,
    #[description = "The amount of messages to delete. Must be less than 100. Defaults to 100."]
    #[min = 1]
    #[max = 100]
    amount: Option<u8>,
) -> Result<(), BotError> {
    ctx.defer_ephemeral().await?;
    let messages = GetMessages::new()
        .after(message_id)
        .limit(amount.unwrap_or(100))
        .execute(ctx.http(), ctx.channel_id())
        .await?;

    for message in &messages {
        message.delete(ctx.http()).await?;
    }

    ctx.say(format!("Deleted {} messages", &messages.len()))
        .await?;

    Ok(())
}
