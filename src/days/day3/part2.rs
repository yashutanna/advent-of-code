use std::ops::{Mul};
use std::fs;
use regex::{Regex};

fn read_file(input_file: &str) -> Vec<(i32, i32)> {
    let mut instructions: String = "".to_owned();

    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .for_each(|raw_line| {
            instructions.push_str(raw_line);
        });

    let multiply_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let do_regex = Regex::new(r"(do\(\))").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut do_parts: Vec<&str> = Vec::new();
    let mut dont_parts_unprocessed: Vec<&str> = Vec::new();
    do_regex
        .split(instructions.as_str())
        .for_each(|part| {
            if part.contains("don't()") {
                dont_parts_unprocessed.push(part);
            } else {
                do_parts.push(part);
            }
        });

    dont_parts_unprocessed.iter().for_each(|part| {
        let dont_parts: Vec<&str> = dont_regex.split(part.to_owned()).map(|split_part| { split_part }).collect();
        do_parts.push(dont_parts[0]);
    });

    let mut final_instructions: String = "".to_owned();
    do_parts.iter().for_each(|value| final_instructions.push_str(value.to_owned()));

    let matches: Vec<(i32, i32)> = multiply_regex
        .captures_iter(final_instructions.as_str())
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            (x, y)
        }).collect();
    matches
}

fn handle_inputs(instructions: Vec<(i32, i32)>) -> i32 {
    let mut total = 0;
    for i in 0..instructions.len() {
        let (x, y) = instructions.get(i).unwrap();
        let result: i32 = x.mul(y);
        total += result;
    }
    total
}

fn executed_timed(instructions: Vec<(i32, i32)>) -> (i32, u128) {
    let start_time = std::time::SystemTime::now();
    let result = handle_inputs(instructions);
    let end_time = std::time::SystemTime::now();
    (result, end_time.duration_since(start_time).unwrap().as_millis())
}

pub(crate) fn run(input_file: &str) {
    let instructions = read_file(input_file);
    let (result, duration) = executed_timed(instructions);
    println!("Result = {}\nCompleted in {}ms", result, duration);
}
