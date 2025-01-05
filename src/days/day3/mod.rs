mod part1;
mod part2;


pub(crate) fn run_part(part: &str, input_file: &String) {
    match part {
        "1" => part1::run(input_file),
        "2" => part2::run(input_file),
        _ => println!("Invalid part provided")
    }
}
