use lits::bytes;

fn main() {
  assert_eq!(bytes!("1 KiB"), 1024);
}
