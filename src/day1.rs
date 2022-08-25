use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| {
            line.parse::<i32>()
                .expect("Input file contains lines that cannot be parsed as integers")
        })
        .collect::<Vec<_>>()
}

#[aoc(day1, part1)]
fn part_one(input: &Vec<i32>) -> i32 {
    let pair = input
        .iter()
        .enumerate()
        .find_map(|(idx, &first)| {
            match input
            .iter()
            .enumerate()
            .find(|(idy, &second)| &idx != idy && (first + second == 2020)){
                Some((_, second)) => Some(first * second),
                None              => None,
            }
        });
    pair.unwrap_or(1)
}

#[aoc(day1, part2)]
fn part_two(input: &Vec<i32>) -> i32 {
    for x in input {
        for y in input {
            if x == y { continue; };
            for z in input {
                if !(x == z || y == z) && (x + y + z == 2020) { return x * y * z; };
            };
        };
    };
    1
}

#[cfg(test)]
#[test]
fn test_find_2020_entries() {
    let test_vec = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(514579, find_2020_entries(&test_vec))
}

#[test]
fn test_find_2020_entries_not_self() {
    let test_vec = vec![1010];

    assert_eq!(1, find_2020_entries(&test_vec))
}

#[test]
fn test_find_2020_entries_double_self() {
    let test_vec = vec![1010, 1010];

    assert_eq!(1020100, find_2020_entries(&test_vec))
}

#[test]
fn test_find_2020_entries_with_three() {
    let test_vec = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(241861950, find_2020_entries_with_three(&test_vec))
}