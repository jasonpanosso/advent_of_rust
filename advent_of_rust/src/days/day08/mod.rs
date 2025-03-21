use super::Day;
use std::collections::{HashMap, HashSet};

pub struct DayStruct;
impl Day for DayStruct {
    type Output = i32;

    fn part_one(&self, input: &str) -> Self::Output {
        let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let mut set: HashSet<(i32, i32)> = HashSet::new();

        let lines: Vec<&str> = input.lines().collect();
        let max_y = lines.len() - 1;
        let max_x = lines[max_y].len() - 1;

        for (y, line) in lines.into_iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != '.' {
                    map.entry(char).or_default().push((x as i32, y as i32));
                }
            }
        }

        for antenna_coordinates in map.values() {
            for (i, coord_one) in antenna_coordinates.iter().enumerate() {
                for coord_two in antenna_coordinates[i + 1..].iter() {
                    let diff = abs_diff(coord_one, coord_two);
                    let antinode_one = compute_antinode(coord_one, coord_two, diff);
                    let antinode_two = compute_antinode(coord_two, coord_one, diff);

                    if coordinate_within_bounds(&antinode_one, max_x, max_y) {
                        set.insert(antinode_one);
                    }
                    if coordinate_within_bounds(&antinode_two, max_x, max_y) {
                        set.insert(antinode_two);
                    }
                }
            }
        }

        set.len() as i32
    }

    fn part_two(&self, input: &str) -> Self::Output {
        let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let mut set: HashSet<(i32, i32)> = HashSet::new();

        let lines: Vec<&str> = input.lines().collect();
        let max_y = lines.len() - 1;
        let max_x = lines[max_y].len() - 1;

        for (y, line) in lines.into_iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != '.' {
                    map.entry(char).or_default().push((x as i32, y as i32));
                }
            }
        }

        for antenna_coordinates in map.values() {
            for (i, coord_one) in antenna_coordinates.iter().enumerate() {
                if antenna_coordinates.len() > 1 {
                    set.insert(*coord_one);
                }

                for coord_two in antenna_coordinates[i + 1..].iter() {
                    let diff = abs_diff(coord_one, coord_two);
                    recursively_push_antinode_in_direction(
                        &mut set, coord_one, coord_two, diff, max_x, max_y,
                    );
                    recursively_push_antinode_in_direction(
                        &mut set, coord_two, coord_one, diff, max_x, max_y,
                    );
                }
            }
        }

        set.len() as i32
    }
}

fn compute_antinode(coord: &(i32, i32), other: &(i32, i32), diff: (i32, i32)) -> (i32, i32) {
    let x = coord.0 + (diff.0 * if coord.0 > other.0 { 1 } else { -1 });
    let y = coord.1 + (diff.1 * if coord.1 > other.1 { 1 } else { -1 });
    (x, y)
}

fn coordinate_within_bounds(coord: &(i32, i32), max_x: usize, max_y: usize) -> bool {
    coord.0 >= 0 && coord.0 <= max_x as i32 && coord.1 >= 0 && coord.1 <= max_y as i32
}

fn abs_diff(coord_one: &(i32, i32), coord_two: &(i32, i32)) -> (i32, i32) {
    (
        coord_one.0.abs_diff(coord_two.0) as i32,
        coord_one.1.abs_diff(coord_two.1) as i32,
    )
}

fn recursively_push_antinode_in_direction(
    set: &mut HashSet<(i32, i32)>,
    coord_one: &(i32, i32),
    coord_two: &(i32, i32),
    diff: (i32, i32),
    max_x: usize,
    max_y: usize,
) {
    let antinode = compute_antinode(coord_one, coord_two, diff);
    if coordinate_within_bounds(&antinode, max_x, max_y) {
        set.insert(antinode);
        recursively_push_antinode_in_direction(set, &antinode, coord_one, diff, max_x, max_y);
    }
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one_example() {
        let expected = 14;

        assert_eq!(DayStruct.part_one(EXAMPLE), expected);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 34;

        assert_eq!(DayStruct.part_two(EXAMPLE), expected);
    }
}
