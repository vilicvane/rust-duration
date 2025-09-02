use lits::duration;

fn main() {
  assert_eq!(duration!("2h").as_secs(), 7200);
}
