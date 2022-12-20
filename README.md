# testx-rs: An extended Rust testcase

The `testx` crate provides the `testx` macro, which is an extended version
of the Rust `test` macro. The key features are:

* The `testx` macro is fully compatible for the Rust `test` macro, all
  tests maked with `#[testx]` (instead of `#[test]`) are executed with
  `cargo-test`.
* Support for a test preparation function.

## Getting started

### Create a `testx` testcase

Mark the testcase with `#[testx]`. Calling `cargo test` will execute the
testcase.

```rust
use testx::testx;

#[testx]
fn sample() {
   assert_eq!(1, 1);
}

// output:
// running 1 test
// test sample ... ok
```

### Prepare data for a testcase with a `setup` function

Provide a function `setup` which prepares and returns some data for your
testcase. Next, your testcase needs one argument, which must match the
return value of the setup function.

A testcase marked with `#[testx]` will first execute the `setup` function
and will pass its return value to your testcase.

```rust
use testx::testx;

fn setup() -> u32 {
    4711
}

#[testx]
pub fn sample(num: u32) {
    assert_eq!(num, 4711);
}
```

## Installation

Put the following line into the `[dev-dependencies]` section of your `Cargo.toml`:

```toml
[dev-dependencies]
testx = "0.1.0"
```
