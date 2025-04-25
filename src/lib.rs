pub mod args;
mod item;

use std::sync::Arc;
use std::{path::PathBuf, process::Command};
use std::error::Error;
use arboard::Clipboard;
use args::Args;
use item::{load_items_from_cache, parse_item_from_json, write_items_to_cache, Item, OpItemSummary};
use log::{error, info};
use skim::prelude::{unbounded, Key, SkimOptionsBuilder};
use skim::{Skim, SkimItemReceiver, SkimItemSender};

type OpTuiResult<T> = Result<T, Box<dyn Error>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    refresh_cache: bool,
    no_cache: bool,
    cache_path: PathBuf,
    vault: String,
}

pub fn get_args(args: Args) -> OpTuiResult<Config> {
    Ok(Config {
        refresh_cache: args.refresh_cache,
        no_cache: args.no_cache,
        cache_path: args.cache_path,
        vault: args.vault,
    })
}

pub fn run(config: Config) -> OpTuiResult<()> {
    info!("{:#?}", config);
    let items = init(config)?;
    let options = SkimOptionsBuilder::default()
        .no_height(true)
        .multi(false)
        .build()
        .expect("Error starting Skim");

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in &items {
        for section in &item.sections {
            tx_item.send(Arc::new(section.clone())).unwrap();
        }
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.
    
    let out = Skim::run_with(&options, Some(rx_item));

    if let Some(o) = out {
        if o.final_key == Key::Enter {
            for item in o.selected_items.iter() {
                info!("Selected: {}", item.output());
                let secret = op_read_reference(item.output().into_owned());
                match secret {
                    Ok(content) => {
                        if !content.trim().is_empty(){
                            copy_to_clipboard(content)?
                        }
                    },
                    Err(e) => error!("Failure getting to secret, {}", e),
                }
            }
        }
    }
    Ok(())
}

fn fetch_and_cache_items(config: &Config) -> OpTuiResult<Vec<Item>> {
    println!("Retrieving items from OP and updating cache");
    let items = fetch_items(config.vault.clone())?;
    write_items_to_cache(&items, config.cache_path.clone())?;
    Ok(items)
}

fn init(config: Config) -> OpTuiResult<Vec<Item>> {
    if config.no_cache {
        println!("Retrieving items from 1password...");
        return fetch_items(config.vault);
    }

    if config.refresh_cache {
        return fetch_and_cache_items(&config);
    }

    match load_items_from_cache(config.cache_path.clone()) {
        Ok(items) => Ok(items),
        Err(e) => {
            if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    println!("Cache file not found. Retrieving items and writing to cache.");

                    let items = fetch_items(config.vault)?;
                    write_items_to_cache(&items, config.cache_path.clone())?;
                    return Ok(items);
                }
            }
            Err(e)
        }
    }
}

fn copy_to_clipboard(content: String) -> OpTuiResult<()> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content.clone())?;

    match clipboard.get_text() {
        Ok(text) if text == content => {
            println!("Secret copied to clipboard! ✅");
            Ok(())
        }
        Ok(text) => {
            error!("Clipboard mismatch: got '{}', expected '{}'", text, content);
            Err("Clipboard content mismatch".into())
        }
        Err(e) => {
            error!("Failed copying to clipboard: {}", e);
            Err(e.into())
        }
    }
}

fn op_read_reference(reference: String) -> OpTuiResult<String> {
    let output = op_command(&["read", &reference])
        .ok_or("Failed to read reference")?;
    Ok(output)
}

fn op_command(args: &[&str]) -> Option<String> {
    info!("op_commands: {:#?}", args);
    Command::new("op")
        .args(args)
        .output()
        .ok()
        .and_then(|result| {
            if result.status.success() {
                Some(String::from_utf8_lossy(&result.stdout).to_string())
            } else {
                error!(
                    "`op` command failed with error: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
                None
            }
        })
}

fn get_vault_args(vault: &str) -> Vec<&str> {
    match vault {
        "favorites" => vec!["item", "list", "--favorite", "--format", "json"],
        "all" => vec!["item", "list", "--format", "json"],
        name => vec!["item", "list", "--vault", name, "--format", "json"],
    }
}

fn fetch_items(vault_name: String) -> OpTuiResult<Vec<Item>> {
    let args = get_vault_args(&vault_name);
    let output = op_command(&args).ok_or("Failed to get item list")?;
    
    let summaries: Vec<OpItemSummary> = serde_json::from_str(&output)
        .map_err(|e| format!("Failed to parse item list JSON: {}", e))?;

    let mut items = Vec::new();

    for summary in summaries {
        let item_output = op_command(&["item", "get", &summary.id, "--format", "json"]);
        match item_output {
            Some(item_json) => {
                match parse_item_from_json(&item_json) {
                    Ok(item) => {
                        items.push(item);
                    }
                    Err(e) => {
                        error!("Failed to parse item '{}': {}", summary.title, e);
                        // Just skip for now and fix fields as we find them
                    }
                }
            }
            None => {
                error!("Failed to get item '{}'", summary.title);
            }
        }
    }
    Ok(items)
}
