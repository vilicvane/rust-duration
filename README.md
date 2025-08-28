# Rust Duration

A simple proc-macro that parses duration strings using [humantime].

## Installation

```sh
cargo add duration
```

## Usage

```rust
use duration::duration;

let interval = duration!("7d");
```

## License

MIT License.

[humantime]: https://crates.io/crates/humantime
