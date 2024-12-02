use super::{Report, parse_input, is_safe_if_one_level_removed};

#[allow(dead_code)]
pub fn run() {
    let result = solve(parse_input());
    println!("result: {}", result);
}

fn solve(reports: Vec<Report>) -> u32 {
    reports.into_iter().filter(|v| is_safe_if_one_level_removed(v)).count() as u32
}
