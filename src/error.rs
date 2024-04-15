use std::fmt::Display;

use poise::serenity_prelude as serenity;

#[derive(Debug)]
pub enum BotError {
    SerenityError(serenity::Error),
}

impl std::error::Error for BotError {}

impl Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<serenity::Error> for BotError {
    fn from(error: serenity::Error) -> Self {
        Self::SerenityError(error)
    }
}
