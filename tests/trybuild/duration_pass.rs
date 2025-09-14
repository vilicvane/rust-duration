use std::time::Duration;

fn main() {
  assert_eq!(lits::duration!("2h"), Duration::from_secs(7200));
  assert_eq!(lits::duration!("30m" * 2), Duration::from_secs(3600));
  assert_eq!(lits::duration!("90s" / 3), Duration::from_secs(30));
  assert_eq!(lits::duration!("2s" * 1.5), Duration::from_secs(3));
  assert_eq!(lits::duration!("1s" / 2.0), Duration::from_millis(500));
}
