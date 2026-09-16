# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Magnetfinder is a Rust CLI that scrapes The Pirate Bay for torrent search results (title, magnet link, size, seeders) and displays them in an interactive terminal table. This is a maintained fork of https://github.com/xel86/magnetfinder. It is search-only: there is no autodownload/torrent-client integration, and no support for other sites (nyaa/YTS were removed).

## Commands

- Build: `cargo build` (release: `cargo build --release`)
- Run: `cargo run -- <args>` (e.g. `cargo run -- -q "search term"`)
- Test (all): `cargo test`
- Test (single): `cargo test piratebay_produces_results` (the only test, in `tests/scraper_tests.rs`)
- Format check: `cargo fmt --check` (apply: `cargo fmt`)
- Lint: `cargo clippy -- -D warnings`

CI (`.github/workflows/ci.yaml`) runs `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` on every PR/push to main.

**Note:** `tests/scraper_tests.rs` makes live HTTP requests to the real piratebay site (no mocking/fixtures) — it's a network-dependent integration test and can fail due to site downtime/HTML changes rather than actual regressions.

## Architecture

- `src/main.rs` — defines the clap `App`/`Arg` CLI schema (`-q/--query`, `--depth`, `--sort`, `--proxy`, `--show`, `--no-interactive`), then delegates everything to `magnetfinder::run`.
- `src/lib.rs` (`run`) — the orchestration entry point:
  1. Resolves `UserParameters` (from CLI args or an interactive query prompt).
  2. Builds a `ureq::Agent` (optionally proxied), wraps it in `Arc`.
  3. Calls `piratebay::query()`, which spawns one thread per page (up to `search_depth`) and fans results into an `mpsc::channel`.
  4. Collects all `Torrent`s from the channel, sorts by size or seeders, truncates to `num_torrents_shown`.
  5. Either renders the interactive table (`interface::display_torrent_table`) and prints the selected magnet(s), or (with `--no-interactive`) prints `title\tmagnet` for every result directly.
- `src/piratebay.rs` — the only scraper module: `query(client, tx, query, depth)` spawns a thread per page and sends `Vec<Torrent>` back over the channel; `fetch_page_results(client, query, page_number) -> Result<Vec<Torrent>, ureq::Error>` does the actual HTTP request + HTML parsing with the `scraper` crate (CSS selectors) — this is what the integration test calls directly. The site's HTML has changed layout before (see git history), so if scraping breaks, re-check the selectors in `get_title`/`get_magnet`/`get_size`/`get_seeders` against a live fetch of a search results page.
- `src/types.rs` — shared domain types: `Torrent`, `Sort`, `Settings`, `UserParameters`. `Torrent::get_size_as_i64` parses human-readable sizes (e.g. `"1.2 GiB"`) into a comparable byte count for sorting.
- `src/settings.rs` — loads/generates `Settings.toml` via the `config` crate from an OS-specific config dir (`directories::ProjectDirs`, e.g. `~/.config/magnetfinder/` on Linux). The only configurable setting is `default_proxy`.
- `src/interface.rs` — houses both CLI-argument parsing into `UserParameters` (`UserParameters::fetch`/`prompt`) and the interactive terminal UI (paginated `comfy-table` display, torrent selection prompt via stdin). `args_present` (checks for `-q/--query`) decides whether to use flag-driven mode vs. the interactive prompt flow.

Cross-cutting: the search query and HTTP client are shared across the per-page scraper threads via `Arc`.
