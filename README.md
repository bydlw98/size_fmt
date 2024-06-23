# size_fmt

This library formats sizes in a human readable format.

---

The Grid API and Implementation is inspired by [`dtolnay/itoa`].

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
