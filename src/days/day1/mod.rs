pub mod part1;
pub mod part2;
use std::fs;

fn read_file(input_file: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|raw_line| {
            let line = String::from(raw_line);
            let value_tuple = line.split("   ")
                .map(String::from)
                .map(|value_string| {
                    let result = value_string.parse::<i32>();
                    return result.unwrap();
                })
                .collect();
            return value_tuple;
        })
        .collect()
}

pub(crate) fn run_part(part: &str, input_file: &String) {
    match part {
        "1" => part1::run(read_file(input_file)),
        "2" => part2::run(read_file(input_file)),
        _ => println!("Invalid part provided")
    }
}
