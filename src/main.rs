mod calling;
mod config;
mod error;
mod logger;

use error::Error;
use logger::LogType;
use poise::serenity_prelude as serenity;
use tokio::sync::{Mutex, RwLock};

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type BotResult<T> = Result<T, Error>;
// Used to store common data across all commands or data the commands might need to access that needs to persist between invocations

pub struct Data {
    logger: Mutex<logger::Logger>,
    calling: RwLock<calling::Calling>,
}

/// Respond with pong
#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!(
        "Pong! Current latency: {}",
        ctx.ping().await.as_millis()
    ))
    .await?;
    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            data.logger
                .lock()
                .await
                .log(
                    LogType::Info,
                    &format!("Bot is ready! Logged in as {}!", data_about_bot.user.name),
                )
                .await?;
        }
        poise::Event::Message { new_message } => {
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
                                .create_webhook(&ctx.http, "Rusty Bot Calling Webhook")
                                .await
                                .unwrap();
                        } else {
                            for hook in channel.webhooks(&ctx.http).await.unwrap() {
                                match hook
                                    .execute(&ctx.http, false, |w| {
                                        w.content(&new_message.content)
                                            .avatar_url(new_message.author.avatar_url().unwrap())
                                            .username(&new_message.author.name)
                                    })
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
        commands: vec![ping(), calling::call()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("--".to_string()),
            ..Default::default()
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(&config::CONFIG.as_ref().token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    logger: Mutex::new(logger::Logger::new().await?),
                    calling: RwLock::new(calling::Calling::new().await),
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run_autosharded()
        .await
        .unwrap();
}
