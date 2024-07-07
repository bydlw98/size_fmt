# size_fmt

[![CICD](https://github.com/bydlw98/size_fmt/actions/workflows/CICD.yml/badge.svg)](https://github.com/bydlw98/size_fmt/actions/workflows/CICD.yml)
[![Crates.io](https://img.shields.io/crates/v/size_fmt)](https://crates.io/crates/size_fmt)
[![Docs.rs](https://img.shields.io/docsrs/size_fmt)](https://docs.rs/size_fmt)
![License](https://img.shields.io/crates/l/size_fmt)

This library formats sizes in a human readable format.

---

The API and Implementation is inspired by [`dtolnay/itoa`].

---

## Features

* Supports no-std.
* No heap allocations.
* Does not go through [`core::fmt::Formatter`].

## Example

```rust
fn main() {
    let mut buffer = size_fmt::Buffer::new();
    let printed = buffer.human_fmt(4096);

    assert_eq!(printed, "4.0K")
}
```

[`core::fmt::Formatter`]: https://doc.rust-lang.org/std/fmt/struct.Formatter.html
[`dtolnay/itoa`]: https://crates.io/crates/itoa
