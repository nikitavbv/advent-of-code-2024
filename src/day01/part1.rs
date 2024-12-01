use super::parse_input;

#[allow(dead_code)]
pub fn run() {
    println!("result is: {}", solve(parse_input()));
}

fn solve(input: Vec<(u64, u64)>) -> u64 {
    let mut first = input.iter().map(|v| v.0).collect::<Vec<_>>();
    let mut second = input.iter().map(|v| v.1).collect::<Vec<_>>();

    first.sort();
    second.sort();

    first.into_iter().zip(second.into_iter()).map(|v| ((v.0 as i64) - (v.1 as i64)).abs() as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(vec![
            (3, 4),
            (4, 3),
            (2, 5),
            (1, 3),
            (3, 9),
            (3, 3),
        ]), 11);
    }
}
