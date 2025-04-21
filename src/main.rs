pub mod args;
mod item;
use clap::Parser;
use log::error;
use op_tui::args::Args;

fn main() {
    env_logger::init();
    if let Err(e) = op_tui::get_args(Args::parse()).and_then(op_tui::run) {
        error!("{}", e);
        std::process::exit(1);
    }
}
