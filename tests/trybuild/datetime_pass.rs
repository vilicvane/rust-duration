use std::time::{Duration, UNIX_EPOCH};

fn main() {
  assert_eq!(
    lits::datetime!("1970-01-01T00:00:00Z")
      .duration_since(UNIX_EPOCH)
      .unwrap(),
    Duration::from_secs(0)
  );
}
