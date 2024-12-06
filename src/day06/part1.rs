use {
    crate::utils::download_input,
    super::{calculate_visited_positions, parse_world},
};

#[allow(dead_code)]
pub fn run() {
    println!("result: {}", calculate_visited_positions(parse_world(&download_input(6))).total_positions());
}
