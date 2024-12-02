use super::Day;

pub struct DayStruct;
impl Day for DayStruct {
    type Output = usize;

    fn part_one(&self, input: &str) -> Self::Output {
        input
            .lines()
            .map(|report| {
                report
                    .split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|report| {
                let levels_ascending = report[0] < report[1];
                report
                    .windows(2)
                    .all(|window| is_level_ok(window[0], window[1], levels_ascending))
            })
            .count()
    }

    fn part_two(&self, input: &str) -> Self::Output {
        input
            .lines()
            .map(|report| {
                report
                    .split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|report| {
                if report
                    .windows(2)
                    .all(|window| is_level_ok(window[0], window[1], report[0] < report[1]))
                {
                    return true;
                }

                for i in 0..report.len() {
                    let mut levels = report[..i].iter().chain(report[(i + 1)..].iter());
                    let mut a = levels.next().unwrap();
                    let mut levels_ascending = None;

                    let report_ok = levels.all(|b| {
                        if a == b || a.abs_diff(*b) > 3 {
                            return false;
                        }

                        match levels_ascending {
                            None => levels_ascending = Some(a > b),
                            Some(true) if a < b => return false,
                            Some(false) if a > b => return false,
                            _ => {}
                        }

                        a = b;
                        true
                    });

                    if report_ok {
                        return true;
                    }
                }

                false
            })
            .count()
    }
}

fn is_level_ok(level_one: i32, level_two: i32, levels_ascending: bool) -> bool {
    // levels can not be equivalent (min difference of one)
    level_one != level_two
    // all levels must be ascending or descending
    && (level_one < level_two) == levels_ascending
    // maximum difference of 3
    && level_one.abs_diff(level_two) <= 3
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one_example() {
        let expected = 2;

        assert_eq!(DayStruct.part_one(EXAMPLE), expected);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 4;

        assert_eq!(DayStruct.part_two(EXAMPLE), expected);
    }
}
