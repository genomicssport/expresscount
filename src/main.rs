mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod threadedhuman;
mod threadedmouse;
use crate::threadedhuman::threadedlengthhuman;
use crate::threadedmouse::threadedlengthmouse;
use async_std::task;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-7-16
*/

fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::ThreadedHuman { count } => {
            let command = task::block_on(threadedlengthhuman(count)).unwrap();
            println!("The command has finished:{:?}", command);
        }
        Commands::ThreadedMouse { count } => {
            let command = task::block_on(threadedlengthmouse(count)).unwrap();
            println!("The command has finished:{:?}", command);
        }
    }
}
