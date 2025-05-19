use crate::cli::commands::run_h3imd3ll_repl;

mod commands;

pub fn run_cli() {
    run_h3imd3ll_repl().unwrap();
}