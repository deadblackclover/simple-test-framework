use colored::*;
use std::time::Instant;

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
        println!("{}", "[ok]".green());
    }
}

// #[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests\n", tests.len());

    let start = Instant::now();

    for test in tests {
        test.run();
    }

    let duration = start.elapsed();

    println!(
        "\nTime elapsed in tests is: {}",
        format!("{:?}", duration).cyan()
    );
}

// #[cfg(test)]
// #[panic_handler]
// pub fn test_panic(info: &PanicInfo) -> () {
//     println!("[failed]\n");
//     println!("Error: {}\n", info);
// }
