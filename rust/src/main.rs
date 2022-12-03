use std::{env, path::Path};

mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: usize = args.get(1).unwrap().parse().unwrap();
    let input_path = Path::new(args.get(2).unwrap());

    match day_num {
        2 => day2::solve(input_path),
        3 => day3::solve(input_path),
        _ => println!("No solution for day {}", day_num),
    };
}
