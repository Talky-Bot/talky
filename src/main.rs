use error::BotError;
use poise::serenity_prelude as serenity;

mod config;
mod error;
mod misc;
mod moderation;

struct Data {}
type Context<'a> = poise::Context<'a, Data, BotError>;

#[tokio::main]
async fn main() {
    let loaded_config = config::Config::new().await;

    let intents = serenity::GatewayIntents::non_privileged();
    let options = poise::FrameworkOptions {
        commands: vec![misc::ping(), moderation::purge::purge()],
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(loaded_config.token, intents)
        .framework(framework)
        .await;

    client.unwrap().start_autosharded().await.unwrap();
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, BotError>,
    _data: &Data,
) -> Result<(), BotError> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("Connected as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}
