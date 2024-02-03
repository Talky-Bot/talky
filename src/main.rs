mod calling;
mod config;
mod error;
mod misc;
mod moderation;

use error::Error;
use poise::serenity_prelude::{self as serenity, CacheHttp};
use tokio::sync::RwLock;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type BotResult<T> = Result<T, Error>;

// Used to store common data across all commands or data the commands might need to access that needs to persist between invocations
pub struct Data {
    calling: RwLock<calling::Calling>,
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("Connected as {}!", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.is_own(&ctx.cache) || new_message.webhook_id.is_some() {
                return Ok(());
            }
            let calling = data.calling.read().await;

            if calling.channel_in_convo(&new_message.channel_id).await {
                let channel = calling
                    .retrieve_channel_id(&new_message.channel_id)
                    .await
                    .unwrap();

                match channel.webhooks(&ctx.http).await {
                    Ok(webhooks) => {
                        if webhooks.is_empty() {
                            channel
                                .create_webhook(
                                    &ctx.http,
                                    serenity::CreateWebhook::new("Rusty Bot Calling Webhook"),
                                )
                                .await
                                .unwrap();
                        } else {
                            for hook in channel.webhooks(&ctx.http).await.unwrap() {
                                match hook
                                    .execute(
                                        &ctx.http,
                                        false,
                                        serenity::ExecuteWebhook::new()
                                            .avatar_url(&new_message.author.avatar_url().unwrap())
                                            .username(&new_message.author.name),
                                    )
                                    .await
                                {
                                    Ok(_) => break,
                                    Err(_) => continue,
                                }
                            }
                        }
                    }
                    Err(_) => {
                        println!("Err??");
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        // Put all commands that need to be registered here
        commands: vec![
            misc::ping(),
            calling::call(),
            moderation::ban::ban(),
            moderation::kick::kick(),
            moderation::unban::unban(),
        ],
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    calling: RwLock::new(calling::Calling::new().await),
                })
            })
        })
        .options(options)
        .build();

    let client = serenity::ClientBuilder::new(
        &config::CONFIG.as_ref().token,
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await;

    client.unwrap().start_autosharded().await.unwrap();
}
