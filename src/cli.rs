use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Magnetfinder",
    version = "1.0",
    author = "bleusakura",
    about = "Scrapes torrent links from The Pirate Bay into the terminal"
)]
pub struct Cli {
    /// search query for desired torrents
    #[arg(short = 'q', long)]
    pub query: Option<String>,

    /// specifies how many pages to search, default is 1
    #[arg(long)]
    pub depth: Option<u32>,

    /// specifies what to sort the torrent table by (size/seeds)
    #[arg(long)]
    pub sort: Option<String>,

    /// sets a proxy to use when making requests to the piratebay
    #[arg(long)]
    pub proxy: Option<String>,

    /// disables any interactive features and simply prints all magnet results found to terminal
    #[arg(long = "no-interactive")]
    pub no_interactive: bool,

    /// limits the amount of torrents results shown, useful with --no-interactive
    #[arg(long = "show")]
    pub num_torrents_shown: Option<usize>,
}
