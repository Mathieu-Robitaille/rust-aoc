use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

struct Password {
    min: usize,
    max: usize,
    challenge_char: char,
    password: String,
}

lazy_static! {
    static ref PATTERN: Regex = {
        let pattern = Regex::new(r"(?x)^
            (\d+) # First capture, min times for validation
            -
            (\d+) # Second capture, max times for validation
            \s
            ([a-z]) # Third capture, character evaluated
            :\s
            (\w+) # Fourth and final capture, the password being tested
        ").unwrap();
        pattern
    };
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<Password> {
    input.lines()
        .map(|x| parse_password(x))
        .collect()
}

fn parse_password(line: &str) -> Password {
    let caps = PATTERN.captures(line).unwrap();

    Password {
        min: str::parse(caps.get(1).unwrap().as_str()).unwrap(),
        max: str::parse(caps.get(2).unwrap().as_str()).unwrap(),
        challenge_char: (caps.get(3).unwrap().as_str()).chars().next().unwrap(),
        password: String::from(caps.get(4).unwrap().as_str()),
    }
}

#[aoc(day2, part1)]
fn part_one(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|pw_to_eval| {
            let pw_char_count = pw_to_eval
                .password
                .chars()
                .filter(|x| x == &pw_to_eval.challenge_char)
                .count();

            pw_to_eval.min <= pw_char_count && pw_to_eval.max >= pw_char_count
        })
        .count()
}

#[aoc(day2, part2)]
fn part_two(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|x| validate_on_position(x))
        .count()
}

fn validate_on_position(ch: &Password) -> bool {
    let mut a: bool = true;
    let mut b: bool = true;
    a ^= char_at_pos(&ch.challenge_char, &ch.min - 1, &ch.password);
    // a ^= ch.password.chars().nth(ch.min - 1).unwrap() == ch.challenge_char;
    b ^= char_at_pos(&ch.challenge_char, &ch.max - 1, &ch.password);
    // b ^= ch.password.chars().nth(ch.max - 1).unwrap() == ch.challenge_char;
    if a ^ b { return true; }
    else { return false; }
}

fn char_at_pos(c: &char, i: usize, p: &String) -> bool {
    p.chars().nth(i).unwrap() == *c
}