use std::collections::HashMap;

use poise::serenity_prelude as serenity;

use crate::{Context, Error};

pub struct Calling {
    // The key is the channel id of channel, the value is the guild id (to make sure that the same server isn't paired togther if the same command is ran in a different channel)
    waiting_servers: HashMap<u64, u64>,
    current_conversations: HashMap<u64, u64>,
}

impl Calling {
    pub async fn new() -> Self {
        Self {
            waiting_servers: HashMap::new(),
            current_conversations: HashMap::new(),
        }
    }

    pub async fn add_to_queue(
        &mut self,
        channel_id: u64,
        guild_id: u64,
        ctx: &Context<'_>,
    ) -> Result<(), ()> {
        if self.contains_channel(&channel_id).await {
            return Err(());
        } else {
            let mut found_channel_id: Option<u64> = None;
            for (key, value) in self.waiting_servers.iter().clone() {
                if value == &guild_id {
                    continue;
                }

                found_channel_id = Some(*key);
                self.current_conversations.insert(*key, channel_id);
                self.current_conversations.insert(channel_id, *key);
                break;
            }

            if found_channel_id.is_some() {
                self.waiting_servers
                    .remove_entry(&found_channel_id.unwrap());
                // Send a message to both channels to let them know that the call is ready
                ctx.say("Call ready! Beware of people trying to scam! They might go as far as being friends with you only to scam you later.").await.expect("Error sending message");

                let channel = serenity::model::id::ChannelId(
                    self.retrieve_channel_id(&channel_id)
                        .await
                        .unwrap()
                        .to_owned(),
                );
                channel.say(&ctx.http(), "Call ready! Beware of people trying to scam! They might go as far as being friends with you only to scam you later.").await.expect("Error sending message");
            } else {
                self.waiting_servers.insert(channel_id, guild_id);
            }
        }
        Ok(())
    }

    pub async fn channel_in_convo(&self, &channel_id: &u64) -> bool {
        self.current_conversations.contains_key(&channel_id)
    }

    pub async fn contains_channel(&self, &channel_id: &u64) -> bool {
        self.waiting_servers.contains_key(&channel_id)
            || self.current_conversations.contains_key(&channel_id)
    }

    pub async fn retrieve_channel_id(&self, channel_id: &u64) -> Option<&u64> {
        self.current_conversations.get(&channel_id)
    }

    pub async fn remove_channel(
        &mut self,
        channel_id: &u64,
        ctx: &Context<'_>,
    ) -> Result<(), Error> {
        match self.current_conversations.remove(channel_id) {
            Some(id) => {
                self.current_conversations.remove(&id);
                ctx.say("Ended Conversation!").await?;

                let channel = serenity::model::id::ChannelId(
                    id
                );
                channel.say(&ctx.http(), "Ended Conversation!").await?;

                Ok(())
            }
            None => match self.waiting_servers.remove(channel_id) {
                Some(_) => {
                    ctx.say("Removed from queue!").await?;
                    Ok(())
                }
                None => {
                    ctx.say("This channel is not in the queue or in a conversation!")
                        .await?;
                    Ok(())
                }
            },
        }
    }
}

/// Call other users on different servers
///
/// Running this command will place you into a queue and if ideal server is found, it enter a call
#[poise::command(slash_command, subcommands("join", "exit"))]
pub async fn call(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let mut calling = ctx.data().calling.write().await;
    match calling
        .add_to_queue(ctx.channel_id().0, ctx.guild_id().unwrap().0, &ctx)
        .await
    {
        Ok(_) => {}
        Err(_) => {
            ctx.say("This channel is alredy in the queue or in a conversation!")
                .await?;
            return Ok(());
        }
    }

    // Create a button that allows the user to leave the queue
    // let exit_queue_button = serenity::CreateButton::default()
    //     .label("Exit Queue")
    //     .style(serenity::ButtonStyle::Danger)
    //     .custom_id("exit_queue")
    //     .to_owned();

    // let action_row = serenity::CreateActionRow::default().add_button(exit_queue_button).to_owned();

    let m = ctx
        .send(
            |m| m.content("Added to queue!"), // .components(|c| c.add_action_row(action_row))
        )
        .await?;

    // let interaction = m
    //     .message()
    //     .await?
    //     .await_component_interaction(ctx)
    //     .timeout(std::time::Duration::from_secs(60))
    //     .author_id(ctx.author().id)
    //     .await
    //     .unwrap();

    // let exit_queue_button = serenity::CreateButton::default()
    //     .label("Exit Queue")
    //     .style(serenity::ButtonStyle::Danger)
    //     .custom_id("exit_queue")
    //     .disabled(true)
    //     .to_owned();

    // let action_row = serenity::CreateActionRow::default().add_button(exit_queue_button).to_owned();

    // m.edit(ctx, |b| {
    //     b.components(|b| b)
    //         .content("Removed from queue!")
    //         .components(|c| c.add_action_row(action_row))
    // }).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn exit(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .calling
        .write()
        .await
        .remove_channel(&ctx.channel_id().0, &ctx)
        .await
        .unwrap();
    Ok(())
}
