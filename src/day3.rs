use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|x| x.chars().collect())
        .collect()
}

#[aoc(day3, part1)]
fn part_one(input: &Vec<Vec<char>>) -> usize {
    let slope = (3, 1);
    count_trees(slope, input)
}

#[aoc(day3, part2)]
fn part_two(forest: &Vec<Vec<char>>) -> usize {
    let slopes = [ (1, 1), (3, 1), (5, 1), (7, 1), (1, 2), ];
    let mut result = 1;
    for slope in slopes {
        result *= count_trees(slope, forest);
    }
    result
}

fn count_trees(slope: (usize, usize), forest: &Vec<Vec<char>>) -> usize {
    let width = forest[0].len();
    let mut trees: usize = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < forest.len() {
        if forest[i][j] == '#' {
            trees += 1;
        }

        // 👀 juuuuuust incase remainder doesnt work
        // j = (j + slope.0).rem_euclid(width); 
        
        i += slope.1;
        j = (j + slope.0) % width;
    }
    trees
}