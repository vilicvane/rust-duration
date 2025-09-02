# Rust Lits

A proc-macro collection that parses human-readable strings at compile time.

## Installation

```sh
cargo add lits
```

## Usage

```rust
use lits::*;

assert_eq!(duration!("7 days"), Duration::from_secs(7 * 24 * 60 * 60));

assert_eq!(bytes!("1 kiB"), 1024);
assert_eq!(bytes!("1 kB"), 1000);
```

## Parsers

- [humantime] for `duration!`.
- [bytesize] for `bytes!`.

## License

MIT License.

[humantime]: https://crates.io/crates/humantime
[bytesize]: https://crates.io/crates/bytesize
