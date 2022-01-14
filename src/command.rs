//! Commands sent from client to server

use displaydoc::Display;
use std::str::FromStr;
use thiserror::Error;

use crate::intcode::{self, IntCode};

/// Any error that occurs on handling `Command`s
#[derive(Debug, Display, Error, PartialEq)]
pub enum Error {
    /// Could not parse message to string
    ParseMessage,
    /// Could not parse string to intcode
    ParseIntCode(#[from] intcode::Error),
}

/// A command parse from a client's message
///
/// # Examples
/// ```
/// use opcode::command::Command;
/// use std::str::FromStr;
///
/// assert_eq!(Command::Help, Command::from_str("help").unwrap());
/// assert_eq!(Command::Quit, Command::from_str("quit").unwrap());
/// assert_eq!([99], Command::from_str("[99]").unwrap().intcode().unwrap()[..]);
/// assert!(Command::from_str("?").is_err());
/// ```
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Display game instructions
    Help,
    /// Close connection
    Quit,
    /// Calcualate an int code
    IntCode(IntCode),
}

impl Command {
    /// Returns the `IntCode` of the command if one.
    pub fn intcode(&self) -> Option<&IntCode> {
        match self {
            Self::IntCode(v) => Some(v),
            _ => None,
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "help" => Ok(Self::Help),
            "quit" => Ok(Self::Quit),
            _ => {
                let code = intcode::parse(s)?;
                Ok(Self::IntCode(code))
            }
        }
    }
}
