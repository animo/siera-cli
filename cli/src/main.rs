mod cli;
mod utils;

use clap::StructOpt;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    println!("one: {:?}", cli.one);
    println!("two: {:?}", cli.two);
}
