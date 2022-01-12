pub mod ws;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    #[clap(short, long, default_value = "ws://localhost:8000/opcode")]
    pub connect_addr: String,
}
