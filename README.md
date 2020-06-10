# assert-panic

[![Latest Version](https://img.shields.io/crates/v/assert-panic.svg)](https://crates.io/crates/assert-panic)
[![docs.rs](https://docs.rs/assert-panic/badge.svg?version=1.0.1)](https://docs.rs/assert-panic/1.0.1/assert_panic/macro.assert_panic.html)

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
    assert_panic!(panic!("at the Disco"), String),
    String,
    starts with "Expected a `String` panic but found one with TypeId { t: ",
);

assert_panic!(
    assert_panic!(panic!("found"), &str, contains "expected"),
    String,
    "Expected a panic containing \"expected\" but found \"found\"",
);

assert_panic!(
    assert_panic!(panic!("found"), &str, starts with "expected"),
    String,
    "Expected a panic starting with \"expected\" but found \"found\"",
);

assert_panic!(
    assert_panic!(panic!(1_usize), usize, 2_usize),
    String,
    "Expected a panic equal to 2 but found 1",
);
```

## Versioning

`assert-panic` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

- The minor version will not reset to 0 on major version changes.  
Consider it the global feature level.
- The patch version will not reset to 0 on major or minor version changes.  
Consider it the global patch level.
- Panic messages originating from `assert-panic` are considered an implementation detail.  
Please only evaluate panic values you have direct control over.
