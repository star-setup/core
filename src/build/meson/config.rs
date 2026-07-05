use crate::{
  build::common::write_mono_repo_config,
  ctx::{IoCtx, RunFlags},
};
use std::path::Path;

/// Generates a root `meson.build` wiring all repositories as subprojects.
/// # Errors
/// Returns an error if the `meson.build` file cannot be written to `mono_dir`.
pub fn create_mono_repo_mesonbuild(
  mono_dir: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  writeln!(io.output, "  Creating Meson configuration").ok();
  write_mono_repo_config(
    mono_dir,
    repos,
    io,
    flags,
    "meson.build",
    |modules| {
      modules
        .iter()
        .map(|m| format!("  '{m}'"))
        .collect::<Vec<_>>()
        .join(",\n")
    },
    |modules_meson| {
      format!(
        "project('star_setup', 'cpp',
  default_options: ['cpp_std=c++20'],
  subproject_dir: 'repos'
)

modules = [
{modules_meson},
]

foreach module : modules
  subproject(module)
endforeach
"
      )
    },
  )
}
