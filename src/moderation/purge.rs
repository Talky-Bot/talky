use poise::serenity_prelude::{
    self as serenity,
    futures::{StreamExt, TryStreamExt},
    Builder, GetMessages,
};

use crate::{error::BotError, Context};

#[poise::command(
    slash_command,
    guild_only,
    subcommands("after", "any", "before"),
    default_member_permissions = "MANAGE_MESSAGES"
)]
pub async fn purge(_ctx: Context<'_>, _amount: u64) -> Result<(), BotError> {
    Ok(())
}

/// Deletes all messages after the given message. Defaults to 100 messages deleted.
#[poise::command(slash_command, ephemeral)]
async fn after(
    ctx: Context<'_>,
    #[description = "The ID of the message to delete after"] message_id: serenity::MessageId,
    #[description = "The amount of messages to delete, up to 1000. Defaults to 100."]
    #[min = 1]
    #[max = 1000]
    amount: Option<u16>,
) -> Result<(), BotError> {
    ctx.defer_ephemeral().await?;
    let mut count: u16 = 0;
    let mut left = amount.unwrap_or(100);
    let mut after_id = message_id;

    while left > 0 {
        let messages = GetMessages::new()
            .limit(left as u8)
            .after(after_id)
            .execute(&ctx.http(), ctx.channel_id())
            .await?;
        after_id = messages.last().unwrap().id;

        for message in messages {
            message.delete(&ctx.http()).await?;
            count += 1;
            left -= 1;
        }
    }

    ctx.say(format!("Deleted {} messages", count)).await?;

    Ok(())
}

/// Deletes the specifed amount of messages. Defaults to 100 messages deleted.
#[poise::command(slash_command, ephemeral)]
async fn any(
    ctx: Context<'_>,
    #[description = "The amount of messages to delete, up to 1000. Defaults to 100."]
    #[min = 1]
    #[max = 1000]
    amount: Option<u16>,
) -> Result<(), BotError> {
    ctx.defer_ephemeral().await?;
    let mut messages = ctx.channel_id().messages_iter(ctx.http()).boxed();
    let mut count = 0;

    for _ in 0..amount.unwrap_or(100) {
        match messages.try_next().await {
            Ok(message) => {
                message.unwrap().delete(&ctx.http()).await?;
                count += 1;
            }
            Err(_) => break,
        }
    }

    ctx.say(format!("Deleted {count} messages")).await?;

    Ok(())
}

/// Deletes all messages before the given message. Defaults to 100 messages deleted.
#[poise::command(slash_command, ephemeral)]
async fn before(
    ctx: Context<'_>,
    #[description = "The ID of the message to delete before"] message_id: serenity::MessageId,
    #[description = "The amount of messages to delete, up to 1000. Defaults to 100."]
    #[min = 1]
    #[max = 1000]
    amount: Option<u16>,
) -> Result<(), BotError> {
    ctx.defer_ephemeral().await?;
    let mut count: u16 = 0;
    let mut left = amount.unwrap_or(100);
    let mut before_id = message_id;

    while left > 0 {
        let messages = GetMessages::new()
            .limit(left as u8)
            .before(before_id)
            .execute(&ctx.http(), ctx.channel_id())
            .await?;
        before_id = messages.first().unwrap().id;

        for message in messages {
            message.delete(&ctx.http()).await?;
            count += 1;
            left -= 1;
        }
    }

    ctx.say(format!("Deleted {} messages", count)).await?;

    Ok(())
}
