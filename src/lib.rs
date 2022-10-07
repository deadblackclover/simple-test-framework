#![feature(custom_test_frameworks)]
#![test_runner(crate::test_frameworks::test_runner)]

#[cfg(test)]
pub mod test_frameworks;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[test_case]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
