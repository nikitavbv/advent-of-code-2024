pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
enum Operation {
    Do,
    Dont,
    Mul,
}

impl Operation {
    fn str_to_look_for(&self) -> &str {
        match self {
            Self::Do => "do()",
            Self::Dont => "don't()",
            Self::Mul => "mul(",
        }
    }
}

pub fn solve(input: impl Into<String>, handle_do: bool, handle_mul: bool) -> u32 {
    let input: String = input.into();

    let mut lookup = [
        Operation::Do,
        Operation::Dont,
        Operation::Mul,
    ].into_iter()
        .map(|op| (op.clone(), input.find(op.str_to_look_for())))
        .filter(|v| v.1.is_some())
        .map(|v| (v.0.clone(), v.1.unwrap() + v.0.str_to_look_for().len()))
        .filter(|v| if handle_do {
            if handle_mul {
                true
            } else {
                match v.0 {
                    Operation::Do => true,
                    _ => false,
                }
            }
        } else {
            match v.0 {
                Operation::Mul => true,
                _ => false,
            }
        })
        .collect::<Vec<_>>();
    lookup.sort_by_key(|v| v.1);

    let (next_op, idx) = match lookup.get(0) {
        Some(v) => v.clone(),
        None => return 0,
    };

    let input = input[idx..].to_owned();
    if input.is_empty() {
        return 0;
    }

    match next_op {
        Operation::Do => return solve(input, true, true),
        Operation::Dont => return solve(input, true, false),
        _ => {},
    }

    let res = parse_number(input);
    let mut input = res.input;
    let first_number = res.number;
    if first_number.is_empty() || first_number.len() > 3 {
        return solve(input, handle_do, handle_mul);
    }
    let first_number: u32 = first_number.parse().unwrap();

    match input.chars().next() {
        Some(v) => if v != ',' {
            return solve(input, handle_do, handle_mul);
        } else {
            input = input[1..].to_string();
        },
        None => return 0,
    };

    let res = parse_number(input);
    let mut input = res.input;
    let second_number = res.number;
    if second_number.is_empty() || second_number.len() > 3 {
        return solve(input, handle_do, handle_mul);
    }
    let second_number: u32 = second_number.parse().unwrap();

    match input.chars().next() {
        Some(v) => if v != ')' {
            return solve(input, handle_do, handle_mul);
        } else {
            input = input[1..].to_string();
        },
        None => return 0,
    }

    first_number * second_number + solve(input, handle_do, handle_mul)
}

struct ParseNumberResult {
    input: String,
    number: String,
}

fn parse_number(mut input: String) -> ParseNumberResult {
    let mut number = String::new();
    for _ in 0..3 {
        let next_char = match input.chars().next() {
            Some(v) => v,
            None => {
                break;
            }
        };
        if !next_char.is_ascii_digit() {
            break;
        }
        input = input[1..].to_string();
        number = format!("{}{}", number, next_char);
    }

    ParseNumberResult {
        input,
        number,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            solve("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", false, true),
            161
        );
    }

    #[test]
    fn test_example_with_instructions() {
        assert_eq!(
            solve("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", true, true),
            48,
        );
    }
}
