use std::io;
use std::process;
use std::sync::Arc;

use clap::ArgMatches;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};

use crate::{Settings, Sort, Torrent, UserParameters};

impl Sort {
    fn new(s: &str) -> Sort {
        match s.to_lowercase().as_str() {
            "size" => Sort::Size,
            _ => Sort::Seeds,
        }
    }
}

impl UserParameters {
    pub fn get_params(args: ArgMatches) -> UserParameters {
        if !args_present(&args) {
            UserParameters::prompt()
        } else {
            UserParameters::fetch(args)
        }
    }

    // handles user interface for providing user settings instead of cmd arguments
    fn prompt() -> UserParameters {
        let settings = match Settings::fetch() {
            Ok(s) => s,
            Err(_) => {
                Settings::generate_settings_file().unwrap_or_else(|err| {
                    eprintln!("error generating new settings file: {}\n", err);
                });
                eprintln!("Generated default Settings.toml");
                Settings::default()
            }
        };

        UserParameters {
            search_query: UserParameters::get_search_query(),
            search_depth: 1,
            sort_preference: Sort::new("seeds"),
            num_torrents_shown: usize::MAX,
            proxy: Arc::new(settings.default_proxy),
            no_interactive: false,
        }
    }

    // parses provided cmd arguments bypassing user interface prompt
    fn fetch(args: ArgMatches) -> UserParameters {
        let config_settings = match Settings::fetch() {
            Ok(s) => s,
            Err(_) => {
                Settings::generate_settings_file().unwrap_or_else(|err| {
                    eprintln!("error generating new settings file: {}\n", err);
                });
                eprintln!("Generated default Settings.toml");
                Settings::default()
            }
        };

        let search_query = Arc::new(String::from(args.value_of("query").unwrap_or_else(|| {
            eprintln!("Must provide a valid search query (-q/--query \"search term\")");
            process::exit(1);
        })));

        let search_depth: u32 = match args.value_of("depth") {
            Some(n) => n.trim().parse().unwrap_or(1),
            None => 1,
        };

        let sort_preference = Sort::new(args.value_of("sort").unwrap_or("seeds"));

        let num_torrents_shown: usize = match args.value_of("num_torrents_shown") {
            Some(n) => n.trim().parse().unwrap_or(usize::MAX),
            None => usize::MAX,
        };

        let proxy = match args.value_of("proxy") {
            Some(p) => Arc::new(String::from(p)),
            None => Arc::new(config_settings.default_proxy),
        };

        UserParameters {
            search_query,
            search_depth,
            sort_preference,
            num_torrents_shown,
            proxy,
            no_interactive: args.is_present("no-interactive"),
        }
    }

    fn get_search_query() -> Arc<String> {
        let mut input = String::new();
        println!("Search query: ");

        io::stdin()
            .read_line(&mut input)
            .expect("io error: couldn't read search query input");

        Arc::new(String::from(input.trim()))
    }
}

pub fn display_torrent_table(torrents: &[Torrent]) -> Vec<&String> {
    let mut torrents_shown: usize = if torrents.len() < 20 {
        torrents.len()
    } else {
        20
    };
    loop {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["#", "Name", "Size", "Seeds"]);

        let table = update_torrent_table(&mut table, &torrents[0..torrents_shown]);
        println!("{}", table);

        if let Some(mag) = prompt_torrent_selection(torrents) {
            return mag;
        }

        torrents_shown = if torrents.len() < torrents_shown + 20 {
            torrents.len()
        } else {
            torrents_shown + 20
        };
    }
}

fn update_torrent_table<'a>(table: &'a mut Table, torrents: &[Torrent]) -> &'a Table {
    for (n, t) in torrents.iter().enumerate() {
        table.add_row(vec![&(n + 1).to_string(), &t.title, &t.size, &t.seeders]);
    }

    table
}

pub fn prompt_torrent_selection(torrents: &[Torrent]) -> Option<Vec<&String>> {
    loop {
        println!("Type 'n' to display 20 more torrents, or select torrent(s) by #:");

        let mut selections = String::new();

        io::stdin()
            .read_line(&mut selections)
            .expect("io error: couldn't read torrent selection input");

        let selections: Vec<&str> = selections.trim().split(' ').collect();
        if selections.is_empty() {
            println!(
                "Please input one or multiple numbers seperated by a space to select torrent(s)"
            );
            continue;
        }

        if selections[0].to_lowercase() == "q" {
            process::exit(1);
        }

        if selections[0].to_lowercase() == "n" {
            return None;
        }

        let magnets = match collect_magnet_links(torrents, &selections) {
            Ok(m) => m,
            Err(s) => {
                println!("{}", s);
                continue;
            }
        };

        return Some(magnets);
    }
}

fn collect_magnet_links<'a>(
    torrents: &'a [Torrent],
    selections: &[&str],
) -> Result<Vec<&'a String>, &'static str> {
    let mut magnets = Vec::new();
    for num_str in selections {
        let num: usize = match num_str.parse() {
            Err(_) => {
                return Err("Only input numbers indicated on the left-most column");
            }
            Ok(num) => num,
        };

        if num > torrents.len() || num == 0 {
            return Err("Input out of range");
        }

        magnets.push(&torrents[num - 1].magnet);
    }
    Ok(magnets)
}

fn args_present(args: &ArgMatches) -> bool {
    args.is_present("query")
}
