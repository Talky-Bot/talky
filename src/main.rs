mod calling;
mod logger;
mod config;

use logger::LogType;
use poise::serenity_prelude as serenity;
use tokio::sync::{RwLock, Mutex};
// Make custom types that are used, dw too much about this
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// Used to store common data across all commands or data the commands might need to access that needs to persist between invocations
struct Data {
    logger: Mutex<logger::Logger>,
    calling: RwLock<calling::Calling>
}

/// Respond with pong
#[poise::command(slash_command, prefix_command)]
async fn ping(
    // Use the custom type defined above
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say(format!("Pong! Current latency: {}", ctx.ping().await.as_millis())).await?;
    Ok(())
}

// These are docstrings, they are used to describe the command, which shows on discord too
/// Call a user on a different server!
#[poise::command(slash_command, prefix_command)]
async fn call(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let mut calling = ctx.data().calling.write().await;
    match calling.add_to_queue(ctx.channel_id().0).await {
        Ok(_) => {},
        Err(_) => {
            ctx.say("This channel is alredy in the queue or in a conversation!").await?;
            return Ok(());
        }
    }

    // Create a button that allows the user to leave the queue
    let exit_queue_button = serenity::CreateButton::default()
        .label("Exit Queue")
        .style(serenity::ButtonStyle::Danger)
        .custom_id("exit_queue")
        .to_owned();
    
    let action_row = serenity::CreateActionRow::default().add_button(exit_queue_button).to_owned();
    

    let _m = ctx.send(|m| m
        .content("Added to queue!")
        // .components(|c| c.add_action_row(action_row))
    ).await?;

    // let interaction = m
    //     .message()
    //     .await?
    //     .await_component_interaction(ctx)
    //     .author_id(ctx.author().id)
    //     .await
    //     .unwrap();

    // m.edit(ctx, |b| {
    //     b.components(|b| b).content("Removed from queue!")
    // }).await?;

    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            data.logger.lock().await.log(LogType::Info, &format!("Bot is ready! Logged in as {}!", data_about_bot.user.name)).await?;
        },
        poise::Event::Message { new_message } => {
            if new_message.is_own(&ctx.cache) {
                return Ok(());
            }
            let channel_string = new_message.channel_id.0;
            let calling = data.calling.read().await;
            
            if calling.channel_in_queue(&channel_string).await {
                let channel = serenity::model::id::ChannelId(calling.retrieve_channel_id(&new_message.channel_id.0).await.unwrap());
                channel.say(&ctx.http, format!("{}: {}", new_message.author.name, new_message.content)).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

// Tokio is a async runtime for Rust
#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        // Put all commands that need to be registered here
        commands: vec![
            ping(),
            call(),
        ],
        // Prefix here
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("--".to_string()),
            // Use the defaults for the rest of the struct
            ..Default::default()
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        // Use the defaults for the rest of the struct
        ..Default::default()
    };

    poise::Framework::builder()
        // Discord Bot Token
        .token(&config::CONFIG.as_ref().token)
        // Initilize the data struct to be used when a command is called
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    logger: Mutex::new(logger::Logger::new().await?),
                    calling: RwLock::new(calling::Calling::new().await)
                })
            })
        })
        // Load the options here
        .options(options)
        // Discord intents
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        // Run the bot sharded, check discord api docs for more info
        .run_autosharded()
        // Keep the program awaiting here as other it will terminate with the bot running
        .await
        // Used to get the result value (if its an error or not)
        .unwrap();
}
