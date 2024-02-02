use poise::serenity_prelude as serenity;
use crate::{BotResult, Context};

/// Ban the specified user
#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The user to ban"] user: serenity::User,
    #[description = "The reason for the ban"] reason: Option<String>,
    #[description = "Days of messages to delete (0-7)"] dod: Option<u8>
) -> BotResult<()> {
    if user.id == ctx.author().id {
        ctx.reply("Can't ban yourself, goofy").await?;
    } else {
        match reason {
            Some(reason) => ctx.guild().unwrap().ban_with_reason(ctx.http(), &user.id, dod.unwrap_or(0), &reason).await?,
            None => ctx.guild().unwrap().ban(ctx.http(), &user.id, dod.unwrap_or(0)).await?,
        }
    
        ctx.reply(format!("Banned user {}\nID:{}", &user.name, &user.id)).await?;
    }
    
    Ok(())
}