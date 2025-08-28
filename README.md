# Rust Duration

A simple proc-macro that parses duration strings using [humantime].

## Installation

```sh
cargo add duration
```

## Usage

```rust
use std::time::Duration;

use duration::duration;

const INTERVAL: Duration = duration!("7d");

fn main() {
  let interval = duration!("7d");
}
```

## License

MIT License.

[humantime]: https://crates.io/crates/humantime
