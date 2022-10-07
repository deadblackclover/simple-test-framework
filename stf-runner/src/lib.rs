use std::time::Instant;

const RED: &str = "\u{001b}[31m";
const GREEN: &str = "\u{001b}[32m";
const CYAN: &str = "\u{001b}[36m";
const RESET: &str = "\u{001b}[0m";

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("{}[ok]{}", GREEN, RESET);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests\n", tests.len());

    let start = Instant::now();

    for test in tests {
        test.run();
    }

    let duration = start.elapsed();

    println!(
        "\nTime elapsed in tests is: {}{:?}{}",
        CYAN, duration, RESET
    );
}
