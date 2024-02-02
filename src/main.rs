mod cli;
mod execute;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    // start discord rpc
    execute::run(args.clone());
}
