# Magnet Finder

This is a fork of https://github.com/xel86/magnetfinder.

This is a CLI application that scrapes torrent results from The Pirate Bay and displays them in a table within your terminal.

![](https://i.imgur.com/piuGz7w.png)

## Usage

Running magnetfinder without any arguments will launch interactive mode, prompting for a search query.

#### Command Line Arguments

- `q, --query <query>`: search query to use
- `--depth <depth>`: specifies how many pages to search through, default is 1
- `--sort <seeds/size>`: allows you to specify if the torrent table is sorted by seeders or size
- `--proxy <proxy url>`: allows you to set a proxy to use when making web requests to the piratebay
- `--show <num>`: truncate list of torrents displayed by the number argument given
- `--no-interactive`: displays all torrents with magnet directly without interacting (--show is useful here)

#### Configuration

Settings.toml (for a default proxy configuration) is located in an OS specific directory:

- `~/.config/magnetfinder/` on Linux
- `/AppData/Roaming/magnetfinder` on Windows
- `/Library/Application Support/magnetfinder/` on MacOS

## Installation

#### Using Cargo

```bash
cargo install magnetfinder
```

#### Download Prebuilt Binaries

Coming soon...
