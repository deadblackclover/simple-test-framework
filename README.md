# simple-test-framework
An example of a simple test framework in Rust

## Usage

```rust
#![feature(custom_test_frameworks)]
#![test_runner(simple_test_framework::test_runner)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[test_case]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
```

## Run example

```bash
cargo test --example simple
```