use std::time::Duration;

fn main() {
  assert_eq!(lits::duration!("2h"), Duration::from_secs(7200));
}
