#![no_std]
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

#[simple_test]
fn it_not_works() {
    let result = add(2, 2);
    assert_eq!(result, 5);
}
