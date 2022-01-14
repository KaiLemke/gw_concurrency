//! OpCode Server

use crate::command::HELP;

pub mod client;
pub mod filter;
pub mod handler;
pub mod ws;

pub use client::{Client, Clients};

/// The buffer size for `mpsc::channel`s for client connections
pub const CLIENT_CONN_SIZE: usize = 42;

/// The servers greeting
pub fn greeting() -> String {
    format!(
        "Welcome to opcode server!\n\n{}\n\n{}\n\n{}\n{}\n{}\n{}\n{}\n\n{}\n",
        HELP[0],
        "To do that you, please connect to ws://127.0.0.1:8000/opcode and I will give you back the modified opcode.",
        HELP[1], HELP[2], HELP[3], HELP[4], HELP[5], HELP[6]
    )
}
