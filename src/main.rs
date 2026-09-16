use clap::Parser;
use magnetfinder::cli::Cli;

fn main() {
    let cli = Cli::parse();
    magnetfinder::run(cli);
}
