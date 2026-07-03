use star_setup::run::run;
use std::{path::PathBuf, process::exit};

fn main() {
  let config_path = PathBuf::from(".star-setup.json");

  if let Err(e) = run(config_path) {
    eprintln!("Error: {e}");
    exit(1);
  }
}
