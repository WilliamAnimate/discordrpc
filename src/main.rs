mod cli;
mod execute;

use std::io::{stdin, stdout, Read, Write};
use crate::cli::Cli;
use clap::Parser;
use std::process::Command;

fn main() {
    let args = Cli::parse();

    // start discord rpc
    execute::run(args.clone());
    println!("Connected!");
    pause()
}

fn pause() { // https://www.reddit.com/r/rust/comments/8tfyof/comment/e177530/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    let mut stdout = stdout();
    stdout.write(b"Do Ctrl + C to exit!").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}