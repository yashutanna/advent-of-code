mod days;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day> [input_file (default=input.txt)]", args[0]);
        eprintln!("Example: {} 1 day_1_input.txt", args[0]);
        return;
    }

    let day = &args[1].to_string();
    let default_file_path = "input.txt".to_string();
    let input_file = args.get(2).unwrap_or(&default_file_path);
    println!("Welcome to advent of code!\nExecuting challenge for day {day}. input file being use is {input_file}" );

    days::day_runner::run_challenge_for_day(day, input_file);
}
