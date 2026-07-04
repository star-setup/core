use crate::common::with_io_output;
use star_setup::{ctx::RunFlags, utils::report_summary};

/* =====     HELPERS     ===== */
fn flags(dry_run: bool, verbose: bool) -> RunFlags {
  RunFlags {
    verbose,
    timing: false,
    dry_run,
  }
}

/* =====     REPORT_SUMMARY     ===== */
#[test]
fn test_report_summary_dry_run_verbose_prints_would() {
  let ((), out) = with_io_output(|io| {
    report_summary(io, flags(true, true), "create thing", "Created thing");
  });
  assert_eq!(out, "  Would create thing\n");
}

#[test]
fn test_report_summary_dry_run_quiet_prints_nothing() {
  let ((), out) = with_io_output(|io| {
    report_summary(io, flags(true, false), "create thing", "Created thing");
  });
  assert_eq!(out, "");
}

#[test]
fn test_report_summary_real_verbose_prints_done() {
  let ((), out) = with_io_output(|io| {
    report_summary(io, flags(false, true), "create thing", "Created thing");
  });
  assert_eq!(out, "  Created thing\n");
}

#[test]
fn test_report_summary_real_quiet_still_prints_done() {
  let ((), out) = with_io_output(|io| {
    report_summary(io, flags(false, false), "create thing", "Created thing");
  });
  assert_eq!(out, "  Created thing\n");
}
