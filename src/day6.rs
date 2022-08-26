use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, HashMap};


#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|line| {
            line.replace("\n", "|")
        })        
        .collect()
}

#[aoc(day6, part1)]
fn part_one(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|item| any_ans(item))
        .sum()
}

#[aoc(day6, part2)]
fn part_two(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|item| common_ans(item))
        .sum()
}

fn any_ans(input: &String) -> usize {
    let set: HashSet<_> = input
        .replace("|", "")
        .chars()
        .collect::<Vec<char>>()
        .drain(..)
        .collect();
    set.len()
}

fn common_ans(input: &String) -> usize {
    let people = input.matches('|').count() + 1;
    let mut hm: HashMap<char, usize> = HashMap::new();
    for x in input.chars() {
        *hm.entry(x).or_default() += 1;
    }

    hm
        .iter()
        .map(|(_, v)| *v)
        .filter(|v| *v == people)
        .collect::<Vec<usize>>()
        .len()
}
