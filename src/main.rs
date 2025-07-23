mod args;
mod exon;
mod threadedhumanexon;
mod threadedmouseexon;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod threadedhuman;
mod threadedmouse;
use crate::threadedhuman::threadedlengthhuman;
use crate::threadedhumanexon::threadedlengthhumanexon;
use crate::threadedmouse::threadedlengthmouse;
use crate::threadedmouseexon::threadedlengthmouseexon;
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
        Commands::ExonThreadedHuman { count } => {
            let command = task::block_on(threadedlengthhumanexon(count)).unwrap();
            println!("The file has been written:{}", command);
        }
        Commands::ExonThreadedMouse { count } => {
            let command = task::block_on(threadedlengthmouseexon(count)).unwrap();
            println!("The file has been written:{}", command);
        }
    }
}
