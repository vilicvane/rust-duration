use std::time::Duration;

use duration::duration;

const DURATION: Duration = duration!("2h");

fn main() {
  assert_eq!(DURATION.as_secs(), 7200);
}
