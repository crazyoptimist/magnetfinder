# Magnet Finder

This is a fork of https://github.com/xel86/magnetfinder.

This is a CLI application that scrapes torrent results from multiple websites and displays them in a table within your terminal.

Supported Websites:
- nyaa 
- piratebay
- YTS

Supported torrent client for autodownloads:
- deluge-console
- transmission-cli
- qbittorrent-cli (*[GitHub](https://github.com/ludviglundgren/qbittorrent-cli))

![](https://i.imgur.com/piuGz7w.png)

## Usage

Running magnetfinder without any arguments will launch interactive mode, prompting for similar information set by flags.

#### Command Line Arguments

- `q, --query <query>`: search query to use
- `n, --nyaa`: scrape nyaa for torrents
- `p, --piratebay`: scrape piratebay for torrents
- `y, --yts`: get torrents from YIFY/YTS
- `a, --all`: scrape all available websites together
- `d, --download`: autodownload the torrent(s) selected
- `--depth <depth>`: specifies how many pages to search through for each website, default is 1
- `--dir <directory>`: directory to download torrent if autodownload was toggled
- `--sort <seeds/size>`: allows you to specify if the torrent table is sorted by seeders or size
- `--proxy <proxy url>`: allows you to set a proxy to use when making web requests to torrent websites & api
- `--show <num>`: truncate list of torrents displayed by the number argument given
- `--no-interactive`: displays all torrents with magnet directly without interacting (--show is useful here)
  
#### Configuration

Settings.toml (for a default configuration, such as download directories & autodownload) is located in an OS specific directory:

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
