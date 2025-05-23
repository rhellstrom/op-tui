# op-tui

op-tui is a fast, terminal-based interface for fuzzy finding between your 1password items and quickly
fetching secrets to your system clipboard.

<img src="https://github.com/rhellstrom/op-tui/blob/main/demo.gif" alt=""/> 

## Quickstart
op-tui is a TUI wrapper that uses the [1password CLI](https://developer.1password.com/docs/cli/get-started/)
to retrieve your 1password items and fuzzy find over them with [Skim](https://github.com/skim-rs/skim).
The selected secret is then fetched and copied to the clipboard using [Arboard](https://github.com/1Password/arboard).

### Installation

To compile the project with [Cargo](https://doc.rust-lang.org/cargo/) run the following command from the root
of the project. This will let you try it out without writing to any files.

```
cargo run -- --no-cache
```

The first run will prompt you for Authorization to 1password and then all items from specified vault will be retrieved - this takes a few seconds depending on what vault you chose(default is all items in all vaults).

This tool requires that the [1password desktop app integration](https://developer.1password.com/docs/cli/get-started/#step-2-turn-on-the-1password-desktop-app-integration
) is configured.

### Design

Relevant items and sections are retrieved using the [item](https://developer.1password.com/docs/cli/reference/management-commands/item/) in JSON format and 
then deserialized using [Serde](https://serde.rs/).

The item metadata is stored either in memory or written to a cache file to improve performance since 
obtaining the item data using the 1password CLI is a time consuming operation (See CLI options below).

Only titles and their secret reference is stored in-memory and in the cache file. When an item is selected the secret
is retrieved using the reference in combination with [op read](https://developer.1password.com/docs/cli/reference/commands/read) and pasted to the clipboard.

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


