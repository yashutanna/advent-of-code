use crate::days;

pub(crate) fn run_challenge_for_day(day: &str, input_file: &String) {
    println!("Running program for day {day} using input file {input_file}" );
    let day_part: Vec<String> = String::from(day)
        .split(".")
        .map(String::from)
        .collect();
    match day_part.get(0).unwrap().as_str() {
        "1" => days::day1::run_part(day_part.get(1).unwrap().as_str(), input_file),
        "2" => days::day2::run_part(day_part.get(1).unwrap().as_str(), input_file),
        _ => println!("Invalid day provided")
    }
}
