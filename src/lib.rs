pub mod cli;
pub mod interface;
pub mod piratebay;
pub mod settings;
pub mod types;

use std::cmp::Reverse;
use std::sync::{Arc, mpsc};

use cli::Cli;
use ureq::Agent;

use types::{Settings, Sort, Torrent, UserParameters};

pub fn run(args: Cli) {
    let user_parameters = UserParameters::get_params(args);

    let client = Arc::new(match build_http_client(&user_parameters.proxy) {
        Ok(client) => client,
        Err(_) => Agent::new_with_defaults(),
    });

    let (tx, rx) = mpsc::channel();
    piratebay::query(
        &client,
        tx,
        &user_parameters.search_query,
        user_parameters.search_depth,
    );

    let mut torrents: Vec<Torrent> = Vec::new();
    for received_torrents in rx {
        torrents.extend(received_torrents);
    }

    match user_parameters.sort_preference {
        Sort::Size => torrents.sort_by_key(|t| Reverse(t.get_size_as_i64())),
        Sort::Seeds => torrents.sort_by_key(|t| Reverse((t.seeders).parse().unwrap_or(0))),
    }

    if torrents.is_empty() {
        eprintln!("No torrents found matching search query");
        return;
    }

    if torrents.len() > user_parameters.num_torrents_shown {
        torrents.truncate(user_parameters.num_torrents_shown);
    }

    if !user_parameters.no_interactive {
        let magnets = interface::display_torrent_table(&torrents);

        for m in magnets {
            println!("{}", m);
        }
    } else {
        for torrent in &torrents {
            println!("{}\t{}", torrent.title, torrent.magnet);
        }
    }
}

fn build_http_client(proxy: &str) -> Result<Agent, Box<ureq::Error>> {
    if proxy.is_empty() {
        Ok(Agent::new_with_defaults())
    } else {
        let config = Agent::config_builder()
            .proxy(Some(ureq::Proxy::new(proxy)?))
            .build();
        Ok(Agent::new_with_config(config))
    }
}
