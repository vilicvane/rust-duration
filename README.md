# Rust Lits

A proc-macro collection that parses human-readable strings at compile time.

## Installation

```sh
cargo add lits
```

## Usage

```rust
use lits::*;

let interval = duration!("7d");

let size = bytes!("1mi");
```

## Parsers

- [humantime] for `duration!`.
- [bytesize] for `bytes!`.

## License

MIT License.

[humantime]: https://crates.io/crates/humantime
[bytesize]: https://crates.io/crates/bytesize
