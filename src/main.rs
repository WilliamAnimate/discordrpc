mod cli;
mod execute;

use crate::cli::Cli;
use clap::Parser;
use std::time::Duration;
use std::thread::sleep;
use std::process::exit;

fn main() {
    let args = Cli::parse();

    // start discord rpc
    execute::run(args.clone());
    println!("Connected!");
    if args.timeout != 0 {
        sleep(Duration::from_secs(args.timeout));

        println!("{}", "Stopping due to timeout...");

        exit(0)
    } else {
        loop {
            // empty `loop {}` wastes CPU cycles
            sleep(Duration::from_secs(9999999));
        }
    }
}