use std::collections::HashMap;

use poise::serenity_prelude::{ChannelId, GuildId};

use crate::{error::Error, BotResult, Context};

pub struct Calling {
    waiting_servers: HashMap<ChannelId, GuildId>,
    current_conversations: HashMap<ChannelId, ChannelId>,
}

enum CallingStatus {
    WaitingInCall,
    AddedToCall(ChannelId),
    RemovedFromQueue,
    EndedCall(ChannelId),
}

impl Calling {
    pub async fn new() -> Self {
        Self {
            waiting_servers: HashMap::new(),
            current_conversations: HashMap::new(),
        }
    }

    async fn add_to_queue(
        &mut self,
        channel_id: ChannelId,
        guild_id: GuildId,
    ) -> BotResult<CallingStatus> {
        if self.contains_channel(&channel_id).await {
            return Err(crate::error::Error::ChannelInQueue);
        } else {
            let mut found_channel_id: Option<ChannelId> = None;
            for (key, value) in self.waiting_servers.iter().clone() {
                if value == &guild_id {
                    continue;
                }

                found_channel_id = Some(*key);
                self.current_conversations.insert(*key, channel_id);
                self.current_conversations.insert(channel_id, *key);
                break;
            }

            if let Some(channel_id) = found_channel_id {
                self.waiting_servers.remove_entry(&channel_id);

                return Ok(CallingStatus::AddedToCall(channel_id));
            } else {
                self.waiting_servers.insert(channel_id, guild_id);
                Ok(CallingStatus::WaitingInCall)
            }
        }
    }

    pub async fn channel_in_convo(&self, &channel_id: &ChannelId) -> bool {
        self.current_conversations.contains_key(&channel_id)
    }

    async fn contains_channel(&self, &channel_id: &ChannelId) -> bool {
        self.waiting_servers.contains_key(&channel_id)
            || self.current_conversations.contains_key(&channel_id)
    }

    pub async fn retrieve_channel_id(&self, channel_id: &ChannelId) -> Option<&ChannelId> {
        self.current_conversations.get(&channel_id)
    }

    async fn remove_channel(&mut self, channel_id: &ChannelId) -> BotResult<CallingStatus> {
        match self.current_conversations.remove(&channel_id) {
            Some(id) => {
                self.current_conversations.remove(&id);
                Ok(CallingStatus::EndedCall(id))
            }
            None => match self.waiting_servers.remove(channel_id) {
                Some(_) => Ok(CallingStatus::RemovedFromQueue),
                None => Err(Error::ChannelNotInQueue),
            },
        }
    }
}

/// Call other users on different servers
///
/// Running this command will place you into a queue and if ideal server is found, it enter a call
#[poise::command(slash_command, subcommands("join", "exit"))]
pub async fn call(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> BotResult<()> {
    let mut calling = ctx.data().calling.write().await;
    match calling
        .add_to_queue(
            ctx.channel_id(),
            match ctx.guild_id() {
                Some(guild) => guild,
                None => {
                    ctx.say("You must use this command in a server!").await?;
                    return Ok(());
                }
            },
        )
        .await
    {
        Ok(status) => match status {
            CallingStatus::WaitingInCall => {
                ctx.say("Added to queue!").await.unwrap();
            }
            CallingStatus::AddedToCall(other_channel) => {
                let message = String::from("Call ready! Beware of people trying to scam! They might go as far as being friends with you only to scam you later.");

                ctx.say(&message).await.unwrap();
                other_channel.say(&ctx.http(), &message).await.unwrap();
            }
            _ => todo!(),
        },
        Err(_) => {
            ctx.say("This channel is alredy in the queue or in a conversation!")
                .await?;
            return Ok(());
        }
    }

    Ok(())
}

/// End the call if you are in one, or exit the queue
#[poise::command(slash_command)]
pub async fn exit(ctx: Context<'_>) -> BotResult<()> {
    match ctx
        .data()
        .calling
        .write()
        .await
        .remove_channel(&ctx.channel_id())
        .await
    {
        Ok(channel_id) => match channel_id {
            CallingStatus::EndedCall(other_channel) => {
                ctx.say("Ended call!").await?;
                other_channel
                    .say(&ctx.http(), "The other server ended the call!")
                    .await?;
            }
            _ => todo!(),
        },
        Err(_) => {
            ctx.say("This channel is not in the queue or conversation!")
                .await?;
        }
    }
    Ok(())
}
