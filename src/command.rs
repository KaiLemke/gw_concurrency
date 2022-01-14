//! Commands sent from client to server

use displaydoc::Display;
use std::str::FromStr;
use thiserror::Error;
use warp::ws::Message;

use crate::{
    intcode::{self, IntCode},
    OpCode,
};

/// Game instructions
// We have to serve it as slice because web socket messages are whitespace trimed
// and we cannot use map in `ws::client_msg()`.
pub const HELP: [&str; 7] = [
    "You can send me an intcode, i.e. a list of integers like '1,0,0,3,2,0,3,6,99'.",
    "Index 0 is an opcode of the following: ",
    "-  1 - add     : Adds together numbers read from two positions and stores a result in a third position.",
    "-  2 - multiply: Does the same as 1 but with multiplication.",
    "- 99 - exit    : Exits the program, i.e. closes the connection immediately.",
    "Multiple opcodes can be sent in one intcode.",
    "If no exit opcode is sent, I will accept further opcodes."
];

/// Reply on a `Command` - a list of `Message`s and option to quit connection
pub type Reply = (Vec<Message>, bool);

/// Any error that occurs on handling `Command`s
#[derive(Debug, Display, Error, PartialEq)]
pub enum Error {
    /// Could not parse message to string
    ParseMessage,
    /// Could not parse string to intcode
    ParseIntCode(#[from] intcode::Error),
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::ParseMessage
    }
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

    /// Creates a reply on a `Command`.
    pub fn reply(&self) -> Reply {
        match self {
            Self::Help => (
                vec![
                    Message::text(HELP[0]),
                    Message::text(HELP[1]),
                    Message::text(HELP[2]),
                    Message::text(HELP[3]),
                    Message::text(HELP[4]),
                    Message::text(HELP[5]),
                    Message::text(HELP[6]),
                ],
                false,
            ),
            Self::Quit => (vec![], true),
            Self::IntCode(ic) => {
                let txt = match OpCode::process(ic.to_vec()) {
                    Ok(ic) => format!("{:?}", ic),
                    Err(err) => format!("Invalid OpCode: {:?}", err),
                };
                (vec![Message::text(txt)], true)
            }
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

impl TryFrom<Message> for Command {
    type Error = Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let s = value.to_str()?;
        let cmd = Self::from_str(s.trim())?;
        Ok(cmd)
    }
}
