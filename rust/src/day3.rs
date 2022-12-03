use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
pub fn solve(input_path: &Path) {
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    let scores = reader
        .lines()
        .map(|line| calculate_line_score(&line.unwrap()));

    let answer_part1: u32 = scores.sum();

    println!("Day 3 part 1 answer: {}", answer_part1);
}

fn calculate_line_score(line: &str) -> u32 {
    let (half1, half2) = line.split_at(line.len() / 2);
    let set1: HashSet<char> = half1.chars().collect();
    let set2: HashSet<char> = half2.chars().collect();

    let common = set1.intersection(&set2);

    common.map(letter_score).sum()
}

fn letter_score(letter: &char) -> u32 {
    if letter.is_lowercase() {
        *letter as u32 - 'a' as u32 + 1
    } else {
        *letter as u32 - 'A' as u32 + 27
    }
}
