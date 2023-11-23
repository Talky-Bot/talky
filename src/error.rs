use std::fmt;

use poise::serenity_prelude;

type DiscordError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum Error {
    ChannelInQueue,
    ChannelNotInQueue,
    IOError(std::io::Error),
    DiscordError(DiscordError),
}

impl From<DiscordError> for Error {
    fn from(error: DiscordError) -> Self {
        Error::DiscordError(error)
    }
}

impl From<serenity_prelude::Error> for Error {
    fn from(error: serenity_prelude::Error) -> Self {
        Error::DiscordError(Box::new(error))
    }
}

// Handles std io errors as well
impl From<tokio::io::Error> for Error {
    fn from(error: tokio::io::Error) -> Self {
        Error::IOError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DiscordError(error) => error.fmt(f),
            Error::ChannelInQueue => todo!(),
            Error::IOError(error) => error.fmt(f),
            Error::ChannelNotInQueue => todo!(),
        }
    }
}
