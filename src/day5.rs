use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

fn process_ticket(input: &str) -> usize {
    let t = input[..7].replace("F", "0").replace("B", "1");
    let s = input[7..].replace("L", "0").replace("R", "1");
    let c = usize::from_str_radix(&t, 2).unwrap();
    let r = usize::from_str_radix(&s, 2).unwrap();

    c * 8 + r
}

#[aoc(day5, part1)]
fn part_one(input: &Vec<String>) -> usize {
    let mut res = 0;
    for x in input {
        let eval = process_ticket(x.as_str());
        res = cmp::max(eval, res);
    }
    res
}

#[aoc(day5, part2)]
fn part_two(input: &Vec<String>) -> usize {
    let mut res = vec![];
    for x in input {
        res.push(process_ticket(x.as_str()));
    }
    res.sort_unstable();
    let x = res
        .windows(2)
        .filter(|&val| !(val[0] + 1 == val[1]) )
        .collect::<Vec<_>>();

    let mut r = 0;
    for e in x.iter() {
        r = e[1] - 1
    }
    r
}

#[cfg(test)]
#[test]
fn test_first_seat() {
    assert_eq!(357, process_ticket("FBFBBFFRLR"));
}