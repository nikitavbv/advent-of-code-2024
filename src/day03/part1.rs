use {
    crate::utils::download_input,
    super::solve,
};

#[allow(dead_code)]
pub fn run() {
    let result = solve(download_input(3), false, true);
    println!("result: {}", result);
}