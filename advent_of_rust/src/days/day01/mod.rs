use super::Day;
use std::collections::HashMap;

pub struct DayStruct;
impl Day for DayStruct {
    type Output = i32;

    fn part_one(&self, input: &str) -> Self::Output {
        let (mut left, mut right) = parse(input);
        left.sort_unstable();
        right.sort_unstable();

        left.iter().zip(&right).map(|(l, r)| (l - r).abs()).sum()
    }

    fn part_two(&self, input: &str) -> Self::Output {
        let (left, right) = parse(input);
        let right_counts: HashMap<i32, i32> =
            right
                .iter()
                .fold(HashMap::with_capacity(right.len()), |mut map, num| {
                    *map.entry(*num).or_insert(0) += 1;
                    map
                });

        left.iter()
            .map(|num| right_counts.get(num).unwrap_or(&0) * num)
            .sum()
    }
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let left = split.next().unwrap().parse::<i32>().unwrap();
            let right = split.next().unwrap().parse::<i32>().unwrap();
            (left, right)
        })
        .unzip()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one_example() {
        let expected = 11;

        assert_eq!(DayStruct.part_one(EXAMPLE), expected);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 31;

        assert_eq!(DayStruct.part_two(EXAMPLE), expected);
    }
}
