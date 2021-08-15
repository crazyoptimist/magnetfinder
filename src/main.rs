use clap::{Arg, App};

fn main() {
    let matches = App::new("Magnetfinder")
        .about("Scrapes torrent links from multiple torrent websites into the terminal")
        .version("1.0")
        .author("bleusakura")
        .arg(
            Arg::with_name("nyaa")
                .help("scrape nyaa for torrents")
                .short("n")
                .long("nyaa")
        )
        .arg(
            Arg::with_name("piratebay")
                .help("scrape piratebay for torrents")
                .short("p")
                .long("piratebay")
        )
        .arg(
            Arg::with_name("all")
                .help("scrape from all available implemented websites")
                .short("a")
                .long("all")
        )
        .arg(
            Arg::with_name("download")
                .help("autodownload's torrent with selected torrent-client")
                .short("d")
                .long("download")
        )
        .arg(
            Arg::with_name("directory")
                .help("directory to download torrent if autodownload toggled")
                .long("dir")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("query")
                .help("search query for desired torrents")
                .long("query")
                .short("q")
                .takes_value(true)
        )
        .get_matches();

    magnetfinder::run(matches);
}