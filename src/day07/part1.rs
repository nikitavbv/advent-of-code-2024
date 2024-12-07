use crate::utils::download_input;

pub struct Equation {
    expected_result: u64,
    parts: Vec<u64>,
}

impl Equation {
    pub fn new(expected_result: u64, parts: Vec<u64>) -> Self {
        Self {
            expected_result,
            parts,
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("result: {}",  solve(parse_input(download_input(7))));
}

pub fn solve(equations: Vec<Equation>) -> u64 {
    equations
        .into_iter()
        .filter(|equation| check_equation(equation))
        .map(|equation| equation.expected_result)
        .sum()
}

pub fn check_equation(equation: &Equation) -> bool {
    is_true_equation(equation.expected_result, equation.parts[0], &equation.parts[1..])
}

fn is_true_equation(target_value: u64, current_value: u64, components: &[u64]) -> bool {
    if components.is_empty() {
        return target_value == current_value
    }

    let next = components[0];
    is_true_equation(target_value, current_value + next, &components[1..])
        || is_true_equation(target_value, current_value * next, &components[1..])
}

fn parse_input(input: String) -> Vec<Equation> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut spl: Vec<_> = line.split(":").collect();
            Equation::new(
                spl.get(0).unwrap().parse().unwrap(),
                spl.get(1).unwrap().split(" ").into_iter()
                    .filter(|number| !number.is_empty())
                    .map(|number| number.trim().parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn check_example_1() {
        assert!(check_equation(&Equation::new(190, vec![10, 19])));
    }

    #[test]
    fn check_example_2() {
        assert!(check_equation(&Equation::new(3267, vec![81, 40, 27])));
    }

    #[test]
    fn check_example_3() {
        assert!(!check_equation(&Equation::new(83, vec![17, 5])));
    }

    #[test]
    fn solve_example() {
        assert_eq!(solve(parse_input(EXAMPLE.to_owned())), 3749);
    }
}
