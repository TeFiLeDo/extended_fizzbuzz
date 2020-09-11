# extended_fizzbuzz ![Build](https://github.com/TeFiLeDo/extended_fizzbuzz/workflows/Build/badge.svg)
Configurable FizzBuzz library.

## Installation
```toml
[dependencies]
extended_fizzbuzz = "1"
```

## Usage
```rust
use extended_fizzbuzz::{fizzbuzz, Matcher};

fn main() {
    let matchers = vec![
        Matcher::new(3, "Fizz").expect("Failed to create `3=Fizz` matcher"),
        Matcher::new(5, "Buzz").expect("Failed to create `5=Buzz` matcher"),
    ];

    fizzbuzz(1, 100, &matchers).expect("FizzBuzzing failed");
}
```

## License
See the [license file](LICENSE) for details.

## Authors
- Adrian Wannenmacher <tfld@tfld.dev>

## Built with
- [thiserror](https://crates.io/crates/thiserror)
- [rand](https://crates.io/crates/rand)
