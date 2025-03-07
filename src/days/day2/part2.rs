use crate::days::day2::part1::{are_all_increasing, are_all_decreasing, are_all_changing_in_defined_range};

fn retry_report_safety_check(report: &Vec<i32>) -> bool {
    let num_levels = report.len();
    for i in 0..num_levels {
        let mut report_copy = report.clone();
        report_copy.remove(i);
        if is_report_safe(report_copy.as_mut(), true){
            return true;
        }
    }
    false
}

fn is_report_safe(report: &Vec<i32>, is_retry: bool) -> bool {
    let all_increasing = are_all_increasing(report);
    let all_decreasing = are_all_decreasing(report);
    let uniform_directional_change = all_decreasing || all_increasing;
    let change_deltas_are_in_range = are_all_changing_in_defined_range(report, 1, 3);
    let safe = uniform_directional_change && change_deltas_are_in_range;
    if !safe && !is_retry {
        return retry_report_safety_check(report);
    }
    safe
}

fn handle_inputs(reports: Vec<Vec<i32>>) -> u32 {
    let mut safe_reports = 0;
    for i in 0..reports.len() {
        let report = reports.get(i).unwrap();
        let is_safe = is_report_safe(report, false);
        if is_safe {
            safe_reports += 1;
        }
    }
    safe_reports
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
