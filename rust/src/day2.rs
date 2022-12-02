extern crate regex;

use regex::Regex;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solve(input_path: &Path) {
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    let scores = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            parse_line(&line).map_err(|e| {
                format!(
                    "Failed to parse line, error = \"{}\", line = \"{}\"",
                    e, line
                )
            })
        })
        .map(|round| round.unwrap())
        .map(|r| r.score());

    let total: i32 = scores.sum();

    println!("Part 2 answer: {}", total);
}

fn parse_line(line: &str) -> Result<Round, String> {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    let captures = re.captures(line).ok_or("Cannot get captures")?;

    let their_choice = Choice::parse(
        captures
            .get(1)
            .ok_or("Cannot find capture group 1")?
            .as_str(),
    )?;

    let required_outcome = Outcome::parse(
        captures
            .get(2)
            .ok_or("Cannot find capture group 2")?
            .as_str(),
    )?;

    let our_choice = their_choice.pick_for_outcome(&required_outcome);

    Ok(Round {
        their_choice: their_choice,
        our_choice: our_choice,
    })
}

#[derive(Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn parse(input: &str) -> Result<Self, String> {
        match input {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(format!("Unknown choice: \"{}\"", input)),
        }
    }

    fn value(&self) -> i32 {
        match *self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn pick_for_outcome(&self, outcome: &Outcome) -> Choice {
        match (self, outcome) {
            (theirs, Outcome::Draw) => *theirs,
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Rock, Outcome::Lose) => Choice::Scissors,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Paper, Outcome::Lose) => Choice::Rock,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Scissors, Outcome::Lose) => Choice::Paper,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn parse(input: &str) -> Result<Self, String> {
        match input {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(format!("Unknown outcome: \"{}\"", input)),
        }
    }

    fn value(&self) -> i32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    their_choice: Choice,
    our_choice: Choice,
}

impl Round {
    fn score(&self) -> i32 {
        self.our_choice.value() + self.outcome().value()
    }

    fn outcome(&self) -> Outcome {
        match *self {
            Round {
                their_choice: Choice::Rock,
                our_choice: Choice::Rock,
            } => Outcome::Draw,
            Round {
                their_choice: Choice::Paper,
                our_choice: Choice::Paper,
            } => Outcome::Draw,
            Round {
                their_choice: Choice::Scissors,
                our_choice: Choice::Scissors,
            } => Outcome::Draw,
            Round {
                their_choice: Choice::Rock,
                our_choice: Choice::Scissors,
            } => Outcome::Lose,
            Round {
                their_choice: Choice::Scissors,
                our_choice: Choice::Rock,
            } => Outcome::Win,
            Round {
                their_choice: Choice::Scissors,
                our_choice: Choice::Paper,
            } => Outcome::Lose,
            Round {
                their_choice: Choice::Paper,
                our_choice: Choice::Scissors,
            } => Outcome::Win,
            Round {
                their_choice: Choice::Paper,
                our_choice: Choice::Rock,
            } => Outcome::Lose,
            Round {
                their_choice: Choice::Rock,
                our_choice: Choice::Paper,
            } => Outcome::Win,
        }
    }
}
