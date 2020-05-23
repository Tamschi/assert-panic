# assert-panic

[![Latest Version](https://img.shields.io/crates/v/assert-panic.svg)](https://crates.io/crates/assert-panic)
[![docs.rs](https://docs.rs/assert-panic/badge.svg?version=1.0.0-preview1)](https://docs.rs/assert-panic/1.0.0-preview1/assert-panic/)

This library provides a macro that asserts that a panic happens, and optionally what (kind of) panic happens.

The simplest version gives you the panic as boxed `Any`.

## Example

```rust
# use std::any::Any;
use assert_panic::assert_panic;

let _: Box<dyn Any + Send + 'static> =
    assert_panic!(panic!("at the Disco"));

assert_panic!(panic!("at the Disco"), &str);

assert_panic!(
    { assert_panic!({}); },
    String,
    starts with "assert_panic! argument did not panic:",
);

assert_panic!(
    assert_panic!(panic!("found"), &str, starts with "expected"),
    String,
    "Expected a panic starting with \"expected\" but found \"found\"",
);

assert_panic!(
    assert_panic!(panic!(1usize), usize, 2usize),
    String,
    "Expected 2 but found 1",
);
```
