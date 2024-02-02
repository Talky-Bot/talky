use poise::serenity_prelude as serenity;

use crate::{BotResult, Context};


/// Removes the ban from specified user
#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The user to unban"] user: serenity::User,
) -> BotResult<()> {
    ctx.guild().unwrap().unban(&ctx.http(), user.id).await?;

    ctx.reply(format!("Unbanned {}", user.id)).await?;

    Ok(())
}