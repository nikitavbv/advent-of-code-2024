use crate::utils::download_input;

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
}

#[derive(Eq, PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

impl Direction {
    pub fn for_numbers(a0: u32, a1: u32) -> Option<Self> {
        Some(if a1 > a0 {
            Direction::Increasing
        } else if a1 == a0 {
            // neither an increase of a decrease are not allowed
            return None;
        } else {
            Direction::Decreasing
        })
    }
}

#[allow(dead_code)]
pub fn run() {
    let input = download_input(2)
        .lines()
        .filter(|v| !v.is_empty())
        .map(|line| Report {
            levels: line.split(" ").map(|v| v.parse().unwrap()).collect(),
        })
        .collect::<Vec<_>>();
    let result = solve(input);
    println!("result: {}", result);
}

fn solve(reports: Vec<Report>) -> u32 {
    reports.into_iter().filter(|v| is_report_safe(v)).count() as u32
}

fn is_report_safe(report: &Report) -> bool {
    if report.levels.len() <= 1 {
        return true;
    }

    let global_direction = match Direction::for_numbers(report.levels[0], report.levels[1]) {
        Some(v) => v,
        None => return false,
    };

    for i in 1..report.levels.len() {
        let direction = match Direction::for_numbers(report.levels[i-1], report.levels[i]) {
            Some(v) => v,
            None => return false,
        };

        if direction != global_direction {
            return false;
        }

        if (report.levels[i-1] as i32 - report.levels[i] as i32).abs() > 3 {
            return false;
        }
    }

    true
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

    #[test]
    fn test_example_1() {
        assert_eq!(is_report_safe(
            &Report {
                levels: vec![7, 6, 4, 2, 1],
            },
        ), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(is_report_safe(
            &Report {
                levels: vec![1, 2, 7, 8, 9],
            },
        ), false);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(is_report_safe(
            &Report {
                levels: vec![9, 7, 6, 2, 1],
            },
        ), false);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(is_report_safe(
            &Report {
                levels: vec![1, 3, 2, 4, 5],
            },
        ), false);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(is_report_safe(
            &Report {
                levels: vec![8, 6, 4, 4, 1],
            },
        ), false);
    }

    #[test]
    fn test_example_6() {
        assert_eq!(is_report_safe(
            &Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ), true);
    }
}
