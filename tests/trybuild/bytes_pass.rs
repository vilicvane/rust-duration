use lits::bytes;

// Use `u32` to make sure it's not annotated with `u64` for better flexibility.
const SIZE: u32 = bytes!("1 KiB");

fn main() {
  assert_eq!(SIZE, 1024u32);
}
