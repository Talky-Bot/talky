use crate::{BotResult, Context};

/// Respond with pong
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> BotResult<()> {
    ctx.say(format!(
        "Pong! Current latency: {}",
        ctx.ping().await.as_millis()
    ))
    .await?;
    Ok(())
}
