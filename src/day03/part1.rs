use crate::utils::download_input;

#[allow(dead_code)]
pub fn run() {
    let result = solve(download_input(3));
    println!("result: {}", result);
}

fn solve(input: impl Into<String>) -> u32 {
    let input: String = input.into();

    let idx = match input.find("mul(") {
        Some(v) => v + 4,
        None => return 0,
    };

    let input = input[idx..].to_string();
    if input.is_empty() {
        return 0;
    }

    let res = parse_number(input);
    let mut input = res.input;
    let first_number = res.number;
    if first_number.is_empty() || first_number.len() > 3 {
        return solve(input);
    }
    let first_number: u32 = first_number.parse().unwrap();

    match input.chars().next() {
        Some(v) => if v != ',' {
            return solve(input);
        } else {
            input = input[1..].to_string();
        },
        None => return 0,
    };

    let res = parse_number(input);
    let mut input = res.input;
    let second_number = res.number;
    if second_number.is_empty() || second_number.len() > 3 {
        return solve(input);
    }
    let second_number: u32 = second_number.parse().unwrap();

    match input.chars().next() {
        Some(v) => if v != ')' {
            return solve(input);
        } else {
            input = input[1..].to_string();
        },
        None => return 0,
    }

    first_number * second_number + solve(input)
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
            solve("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

}
