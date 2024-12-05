use super::Day;
use std::collections::{HashMap, HashSet};

pub struct DayStruct;
impl Day for DayStruct {
    type Output = i32;

    fn part_one(&self, input: &str) -> Self::Output {
        let mut output = 0;
        let (page_ordering_rules_map, updates) = parse(input);

        for line in updates.iter() {
            let mut seen: HashSet<i32> = HashSet::new();
            let mut valid_line = true;

            for num in line.iter() {
                if seen.contains(num) {
                    continue;
                }

                if let Some(must_come_after) = page_ordering_rules_map.get(num) {
                    if must_come_after.iter().any(|val| seen.contains(val)) {
                        valid_line = false;
                        break;
                    }
                }

                seen.insert(*num);
            }

            if valid_line {
                output += line[line.len() / 2]
            }
        }

        output
    }

    fn part_two(&self, input: &str) -> Self::Output {
        let mut output = 0;
        let (page_ordering_rules_map, mut updates) = parse(input);

        for line in updates.iter_mut() {
            let mut i = 0;
            let mut valid_line = true;

            while i < line.len() {
                if let Some(must_come_after) = page_ordering_rules_map.get(&line[i]) {
                    if let Some(index) = must_come_after
                        .iter()
                        .find_map(|&val| line[..i].iter().position(|&n| n == val))
                    {
                        valid_line = false;

                        let removed = line.remove(i);
                        line.insert(index, removed);
                        i = index;

                        continue;
                    }
                }

                i += 1;
            }

            if !valid_line {
                output += line[line.len() / 2]
            }
        }

        output
    }
}

fn parse(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let (page_ordering_rules_raw, update_raw) = input.split_once("\n\n").unwrap();

    let mut page_ordering_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    page_ordering_rules_raw.lines().for_each(|line| {
        let mut parts = line.split('|');
        let key = parts.next().unwrap().parse::<i32>().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        page_ordering_rules.entry(key).or_default().insert(value);
    });

    let updates: Vec<Vec<i32>> = update_raw
        .lines()
        .map(|s| {
            s.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (page_ordering_rules, updates)
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one_example() {
        let expected = 143;

        assert_eq!(DayStruct.part_one(EXAMPLE), expected);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 123;

        assert_eq!(DayStruct.part_two(EXAMPLE), expected);
    }
}
