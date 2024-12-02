use super::{Report, parse_input, is_report_safe};

#[allow(dead_code)]
pub fn run() {
    let result = solve(parse_input());
    println!("result: {}", result);
}

fn solve(reports: Vec<Report>) -> u32 {
    reports.into_iter().filter(|v| is_report_safe(v)).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_example() {
        assert_eq!(
            solve(vec![
                Report {
                    levels: vec![7, 6, 4, 2, 1],
                },
                Report {
                    levels: vec![1, 2, 7, 8, 9],
                },
                Report {
                    levels: vec![9, 7, 6, 2, 1],
                },
                Report {
                    levels: vec![1, 3, 2, 4, 5],
                },
                Report {
                    levels: vec![8, 6, 4, 4, 1],
                },
                Report {
                    levels: vec![1, 3, 6, 7, 9],
                },
            ]),
            2
        )
    }
}
