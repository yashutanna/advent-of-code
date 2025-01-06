mod part1;

use std::fs;

fn read_file(input_file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|raw_report| {
            return raw_report.chars().collect()
        })
        .collect()
}

pub(crate) fn run_part(part: &str, input_file: &String) {
    match part {
        "1" => part1::run(read_file(input_file)),
        _ => println!("Invalid part provided")
    }
}
