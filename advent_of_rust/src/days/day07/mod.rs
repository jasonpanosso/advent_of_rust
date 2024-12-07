use super::Day;

enum Operator {
    Multiply,
    Add,
    Concat,
}

impl Operator {
    fn execute(&self, l: i64, r: i64) -> i64 {
        match self {
            Operator::Multiply => l * r,
            Operator::Add => l + r,
            Operator::Concat => format!("{}{}", l, r)
                .parse::<i64>()
                .expect("concatenated value exceeds i64"),
        }
    }
}

pub struct DayStruct;
impl Day for DayStruct {
    type Output = i64;

    fn part_one(&self, input: &str) -> Self::Output {
        solve(input, &[Operator::Add, Operator::Multiply])
    }

    fn part_two(&self, input: &str) -> Self::Output {
        solve(
            input,
            &[Operator::Add, Operator::Multiply, Operator::Concat],
        )
    }
}

fn solve(input: &str, operands: &[Operator]) -> i64 {
    input
        .lines()
        .map(|l| {
            let (goal_str, nums_str) = l
                .split_once(':')
                .expect("malformed line without ':' in input str");
            let goal = goal_str.parse::<i64>().expect("non i64 in input goal");
            let nums: Vec<i64> = nums_str
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect("non i64 in input operands"))
                .collect();

            if has_solution(goal, &nums, 1, nums[0], operands) {
                goal
            } else {
                0
            }
        })
        .sum()
}

fn has_solution(
    goal: i64,
    nums: &[i64],
    index: usize,
    current: i64,
    operands: &[Operator],
) -> bool {
    if index == nums.len() {
        return current == goal;
    }

    let next_num = nums[index];
    for operand in operands {
        if has_solution(
            goal,
            nums,
            index + 1,
            operand.execute(current, next_num),
            operands,
        ) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one_example() {
        let expected = 3749;

        assert_eq!(DayStruct.part_one(EXAMPLE), expected);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 11387;

        assert_eq!(DayStruct.part_two(EXAMPLE), expected);
    }
}
