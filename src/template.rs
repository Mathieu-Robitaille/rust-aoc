use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(dayX)]
fn parse_input_dayX(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| {
            line.parse::<i32>()
                .expect("Input file contains lines that cannot be parsed as integers")
        })
        .collect::<Vec<_>>()
}

#[aoc(dayX, part1)]
fn part_one(input: b) -> b {

}

#[aoc(dayX, part2)]
fn part_two(input: b) -> b {

}
