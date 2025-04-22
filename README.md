# op-tui

op-tui is a fast, terminal-based interface for fuzzy finding between your 1password items and quickly
fetching secrets to your system clipboard.

<img src="https://github.com/rhellstrom/op-tui/blob/main/demo.gif" alt=""/> 

## Quickstart
op-tui is a TUI wrapper that uses the [1password CLI](https://developer.1password.com/docs/cli/get-started/)
to retrieve your 1password items and fuzzy find over them with [Skim](https://github.com/skim-rs/skim).
The selected secret is then fetched and copied to the clipboard using [Arboard](https://github.com/1Password/arboard).

### Installation

To compile and run it using [Cargo](https://doc.rust-lang.org/cargo/) from the root of the project:

```
cargo run --release
```

The first run will prompt you for Authorization to 1password and then all items from specified vault will be retrieved - this takes a few seconds depending on what vault you chose(default is all items in all vaults).

### Design

Relevant items and their fields and sections are retrieved using the [item subcommands list and get](https://developer.1password.com/docs/cli/reference/management-commands/item/)
in JSON format. Relevant fields are then deserialized using [Serde](https://serde.rs/).

The item metadata is stored either in memory or written to a cache file to improve performance since 
obtaining the item data using the 1password CLI is a time consuming operation. 
See CLI options below.

We store no actual secrets in-memory or the in cache file. We only store the titles and the associated secret reference.
Once we've selected an item the reference of that items is returned. [op read](https://developer.1password.com/docs/cli/reference/commands/read)
is then used to retrieve the password/secret and paste it to the clipboard.

This tool requires that the [1password desktop app integration](https://developer.1password.com/docs/cli/get-started/#step-2-turn-on-the-1password-desktop-app-integration
) is configured.

For information on the Authorization and Security model of this integration see: https://developer.1password.com/docs/cli/app-integration-security

### CLI Options
```
Usage: op-tui [OPTIONS] [CACHE_PATH]

Arguments:
  [CACHE_PATH]  Path to file for caching op items. Unless no_cache is set, op-tui will attempt to load items from this file. If no file is found op-tui will attemmpt to retrieve items from 1password and then cache them to this file [default: ~/.cache/op-tui/items.json]

Options:
  -r, --refresh-cache  Retrieves items from 1password and update the cache file
      --no-cache       Do not load or write items retrieved from 1password to cache file
      --vault <VAULT>  Vault name, `favorites`, or `all` [default: all]
  -h, --help           Print help
  -V, --version        Print version

```
## License
