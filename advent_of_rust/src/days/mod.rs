pub trait Day {
    type Output: std::fmt::Display;

    fn part_one(&self, input: &str) -> Self::Output;
    fn part_two(&self, input: &str) -> Self::Output;
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

macros::generate_days_enum!();
