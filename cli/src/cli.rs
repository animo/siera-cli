use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long)]
    pub two: String,
    #[clap(long)]
    pub one: String,
}
