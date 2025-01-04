fn handle_inputs(inputs: Vec<Vec<i32>>) -> u32 {
    let mut lefts: Vec<&i32> = Vec::new();
    let mut rights: Vec<&i32> = Vec::new();

    inputs.iter().for_each(|input| {
        lefts.push(input.get(0).unwrap());
        rights.push(input.get(1).unwrap());
    });

    lefts.sort();
    rights.sort();
    let mut result = 0;
    for i in 0..lefts.len() {
        result += lefts[i].abs_diff(rights[i].to_owned())
    }
    result
}

fn executed_timed(inputs: Vec<Vec<i32>>) -> (u32, u128) {
    let start_time = std::time::SystemTime::now();
    let result = handle_inputs(inputs);
    let end_time = std::time::SystemTime::now();
    (result, end_time.duration_since(start_time).unwrap().as_millis())
}

pub(crate) fn run(inputs: Vec<Vec<i32>>) {
    let (result, duration) = executed_timed(inputs);
    println!("Result = {}\nCompleted in {}ms", result, duration);
}
