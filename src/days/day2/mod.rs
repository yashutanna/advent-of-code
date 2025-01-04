mod part1;
mod part2;

use std::fs;

fn read_file(input_file: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|raw_report| {
            let report = String::from(raw_report);
            let report_levels = report.split(" ")
                .map(String::from)
                .map(|value_string| {
                    let result = value_string.parse::<i32>();
                    return result.unwrap();
                })
                .collect();
            return report_levels;
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
