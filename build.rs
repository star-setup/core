use embed_manifest::{embed_manifest, new_manifest};
use embed_manifest::manifest::ExecutionLevel;

fn main() {
  if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
    embed_manifest(
      new_manifest("masonlet.star-setup")
        .requested_execution_level(ExecutionLevel::AsInvoker),
    )
    .expect("failed to embed manifest");
  }
}
