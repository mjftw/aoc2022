extern crate itertools;

use itertools::Itertools;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solve(input_path: &Path) {
    let file = File::open(input_path).unwrap();

    let answer_part2 = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.reduce(|a, b| &a & &b).unwrap())
        .map(|common| common.into_iter().map(|l| letter_score(&l)).sum::<u32>())
        .sum::<u32>();

    println!("Day 3 part 2 answer: {}", answer_part2);
}

fn letter_score(letter: &char) -> u32 {
    if letter.is_lowercase() {
        *letter as u32 - 'a' as u32 + 1
    } else {
        *letter as u32 - 'A' as u32 + 27
    }
}
