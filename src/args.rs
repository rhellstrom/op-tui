use std::{env, fs, path::PathBuf};
use clap::{ArgAction, Parser};

fn get_default_cache_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("HOME not set");
    let mut path = PathBuf::from(home_dir);
    path.push(".cache");
    path.push("op-tui");
    fs::create_dir_all(&path).expect("Failed to create config directory");
    path.push("items.json");
    path
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Retrieves items from 1password and update the cache file
    #[arg(short = 'r', long = "refresh-cache", action = ArgAction::SetTrue, default_value_t = false, group = "init")]
    pub refresh_cache: bool,

    /// Do not load or write items retrieved from 1password to cache file
    #[arg(long = "no-cache", action = ArgAction::SetTrue, group = "init")]
    pub no_cache: bool,

    /// Path to file for caching op items. Unless no_cache is set, op-tui will attempt to load items from this
    /// file. If no file is found op-tui will attemmpt to retrieve items from 1password and then
    /// cache them to this file
    #[arg(default_value=get_default_cache_path().into_os_string())]
    pub cache_path: PathBuf,

    /// Name of the vault to retrieve items from.
    /// If a cache file is already initialized this command will need to be run
    /// with --refresh-cache or --no-cache
    #[arg(long, default_value = "all", value_name = "VAULT", help = "Vault name, `favorites`, or `all`")]
    pub vault: String,
}
