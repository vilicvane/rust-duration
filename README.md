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
assert_eq!(duration!("1m" * 1.2), Duration::from_secs(72));
assert_eq!(duration!("2s" / 20), Duration::from_millis(100));

assert_eq!(datetime!("1970-01-01T00:00:00Z"), UNIX_EPOCH);

assert_eq!(bytes!("1 kiB"), 1024);
assert_eq!(bytes!("1 kB"), 1000);
```

## Features

- [humantime]
  - `duration!()` ≈ `humantime::parse_duration()`
  - `datetime!()` ≈ `humantime::parse_rfc3339_weak()`
- [bytesize]
  - `bytes!()` ≈ `<FromStr>::parse::<bytesize::ByteSize>().as_u64()`

> All features are enabled by default.

## License

MIT License.

[humantime]: https://crates.io/crates/humantime
[bytesize]: https://crates.io/crates/bytesize
