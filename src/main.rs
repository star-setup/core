//! Entry point. Parses arguments, loads config, and dispatches to the appropriate command handler.

use star_setup::run::run;
use std::path::PathBuf;

fn main() {
  let config_path = PathBuf::from(".star-setup.json");

  if let Err(e) = run(config_path) {
    eprintln!("Error: {e}");
    std::process::exit(1);
  }
}
