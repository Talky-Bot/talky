use poise::serenity_prelude as serenity;

use crate::{BotResult, Context};

/// Kick the specified user
#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick"] user: serenity::User,
    #[description = "The reason for the kick"] reason: Option<String>,
) -> BotResult<()> {
    if user.id == ctx.author().id {
        ctx.reply("Can't kick yourself, goofy").await?;
    } else {
        match reason {
            Some(reason) => ctx.guild().unwrap().kick_with_reason(&ctx.http(), &user.id, &reason).await?,
            None => ctx.guild().unwrap().kick(&ctx.http(), &user.id).await?,
        }
    }

    Ok(())
}