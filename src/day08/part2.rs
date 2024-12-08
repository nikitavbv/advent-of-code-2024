use {
    crate::utils::download_input,
    super::{solve, parse_map},
};

#[allow(dead_code)]
pub fn run() {
    println!("result: {}", solve(&parse_map(&download_input(8)), true));
}
