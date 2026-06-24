//! Entry point. Parses arguments, loads config, and dispatches to the appropriate command handler.

use star_setup::run::run;

fn main() {
  if let Err(e) = run() {
    eprintln!("Error: {e}");
    std::process::exit(1);
  }
}
