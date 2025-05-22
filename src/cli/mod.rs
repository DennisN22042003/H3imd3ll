use crate::cli::commands::run_h3imd3ll_repl;

mod commands;
mod utils;

pub fn run_cli() {
    run_h3imd3ll_repl().unwrap();
}