use crate::{error::BotError, Context};

/// Respond with the heartbeat latency. Returns 0 if the bot has just started
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), BotError> {
    ctx.say(format!(
        "Heartbeat Latency: {}ms",
        ctx.ping().await.as_millis()
    ))
    .await?;
    Ok(())
}
