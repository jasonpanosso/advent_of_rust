pub trait Day {
    type Output: std::fmt::Debug;

    fn part_one(&self, input: &str) -> Self::Output;
    fn part_two(&self, input: &str) -> Self::Output;
}

macros::generate_day_modules!();
macros::generate_days_enum!();
