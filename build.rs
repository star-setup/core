use embed_manifest::manifest::ExecutionLevel;
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
  if std::env::var_os("CARGO_CFG_WINDOWS").is_some() && cfg!(target_os = "windows") {
    embed_manifest(
      new_manifest("masonlet.star-setup").requested_execution_level(ExecutionLevel::AsInvoker),
    )
    .expect("failed to embed manifest");
  }
}
