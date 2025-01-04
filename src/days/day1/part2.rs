use std::collections::HashMap;
use std::ops::Mul;

fn handle_inputs(inputs: Vec<Vec<i32>>) -> i32 {
    let mut lefts: Vec<i32> = Vec::new();
    let mut right_map: HashMap<&i32, i32> = HashMap::new();

    inputs.iter().for_each(|input| {
        lefts.push(input.get(0).unwrap().to_owned());
        let right_value = input.get(1).unwrap();
        let current_entry = right_map.entry(right_value).or_insert(i32::from(0));
        *current_entry += 1;
    });


    let mut result = 0;
    for i in 0..lefts.len() {
        let left_value = lefts[i];
        let number_of_occurrence = right_map.get(&left_value);
        if number_of_occurrence.is_none() {
            continue;
        }
        result += lefts[i].mul(number_of_occurrence.unwrap());
    }
    result.to_owned()
}

fn executed_timed(inputs: Vec<Vec<i32>>) -> (i32, u128) {
    let start_time = std::time::SystemTime::now();
    let result = handle_inputs(inputs);
    let end_time = std::time::SystemTime::now();
    (result, end_time.duration_since(start_time).unwrap().as_millis())
}

pub(crate) fn run(inputs: Vec<Vec<i32>>) {
    let (result, duration) = executed_timed(inputs);
    println!("Result = {}\nCompleted in {}ms", result, duration);
}
