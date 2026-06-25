#[macro_export]
macro_rules! time {
  ($timing:expr, $output:expr, $msg:expr, $block:expr) => {{
    let t = std::time::Instant::now();
    let result = $block;
    if $timing {
      let _ = std::io::Write::write_fmt(
        $output,
        format_args!("  [timing] {}: {:.2?}\n", $msg, t.elapsed()),
      );
    }
    result
  }};
}
