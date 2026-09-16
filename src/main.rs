use clap::{App, Arg};

fn main() {
    let matches = App::new("Magnetfinder")
        .about("Scrapes torrent links from The Pirate Bay into the terminal")
        .version("1.0")
        .author("bleusakura")
        .arg(
            Arg::with_name("query")
                .help("search query for desired torrents")
                .long("query")
                .short("q")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("depth")
                .help("specifies how many pages to search, default is 1")
                .long("depth")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sort")
                .help("specifies what to sort the torrent table by (size/seeds)")
                .long("sort")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("proxy")
                .help("sets a proxy to use when making requests to the piratebay")
                .long("proxy")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no-interactive")
                .help("disables any interactive features and simply prints all magnet results found to terminal")
                .long("no-interactive")
        )
        .arg(
            Arg::with_name("num_torrents_shown")
                .help("limits the amount of torrents results shown, useful with --no-interactive")
                .long("show")
                .takes_value(true),
        )
        .get_matches();

    magnetfinder::run(matches);
}
