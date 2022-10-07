# simple-test-framework

An example of a simple test framework in Rust

## Usage

```rust
#![feature(custom_test_frameworks)]
#![test_runner(stf_runner::test_runner)]

use stf_macro::simple_test;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[simple_test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
```

## Sub-projects

- [stf-macro](https://github.com/deadblackclover/simple-test-framework/tree/master/stf-macro) ![crates.io](https://img.shields.io/crates/v/stf-macro.svg)

- [stf-runner](https://github.com/deadblackclover/simple-test-framework/tree/master/stf-runner) ![crates.io](https://img.shields.io/crates/v/stf-runner.svg)
