#[macro_export]
macro_rules! time {
  ($timing:expr, $output:expr, $msg:expr, $block:expr) => {{
    let t = std::time::Instant::now();
    let result = $block;
    if $timing {
      std::io::Write::write_fmt(
        $output,
        format_args!("  [timing] {}: {:.2?}\n", $msg, t.elapsed()),
      )
      .ok();
    }
    result
  }};
}
