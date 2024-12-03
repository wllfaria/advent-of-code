pub mod day1;
pub mod day2;
pub mod day3;

use std::fmt::Display;

pub fn run_day(day: u8, part: Option<u8>) -> Result<(), String> {
    match day {
        1 => day1::run(part),
        2 => day2::run(part),
        3 => day3::run(part),
        _ => panic!("day {day} is not solved"),
    };

    Ok(())
}

pub fn timed<F, T>(f: F, input: &str, part: u8) -> T
where
    T: Display,
    F: FnOnce(&str) -> T,
{
    let start = std::time::Instant::now();
    let result = f(input);
    let elapsed = start.elapsed();
    println!("Part {part} finished in {elapsed:?} with result {result}");
    result
}
