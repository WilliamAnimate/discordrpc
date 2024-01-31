mod cli;
mod execute;

use crate::cli::Cli;
use clap::Parser;
use std::process::Command;

fn main() {
    let args = Cli::parse();

    // start discord rpc
    execute::run(args.clone());
    println!("Connected!");
}