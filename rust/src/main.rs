use std::{env, path::Path};

mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = Path::new(args.get(1).unwrap());

    day2::solve(input_path);
}
