use {
    crate::utils::download_input,
    super::{solve, parse_input},
};

#[allow(dead_code)]
pub fn run() {
    println!("result: {}",  solve(parse_input(download_input(7)), true));
}
