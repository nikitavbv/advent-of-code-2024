use {
    num_bigint::BigUint,
    num_traits::{ops::checked::CheckedMul, FromPrimitive},
};

#[derive(Eq, PartialEq)]
struct Stone {
    number: BigUint,
}

struct Stones {
    stones: Vec<Stone>,
}

impl Stone {
    pub fn new(number: BigUint) -> Self {
        Self {
            number,
        }
    }

    fn is_even_number_of_digits(&self) -> bool {
        self.number.to_string().len() % 2 == 0
    }
}

impl Stones {
    pub fn new(stones: Vec<Stone>) -> Self {
        Self {
            stones,
        }
    }

    fn next(&self) -> Self {
        let mut result = Vec::new();

        for stone in &self.stones {
            if stone.number == BigUint::ZERO {
                result.push(Stone::new(BigUint::from_u32(1).unwrap()));
            } else if stone.is_even_number_of_digits() {
                let str = stone.number.to_string();
                result.push(Stone::new(str[0..str.len() / 2].parse().unwrap()));
                result.push(Stone::new(str[str.len()/2..].parse().unwrap()));
            } else {
                result.push(Stone::new(stone.number.checked_mul(&BigUint::from_u32(2024).unwrap()).unwrap()));
            }
        }

        Self::new(result)
    }
}

fn parse_stones(input: &str) -> Stones {
    Stones::new(
        input.replace("\n", "")
            .split(" ")
            .map(|v| Stone::new(v.parse().unwrap()))
            .collect()
    )
}

pub mod part1 {
    use {
        crate::utils::download_input,
        super::parse_stones,
    };

    #[allow(dead_code)]
    pub fn run() {
        println!("result: {}", solve(&download_input(11)));
    }

    pub fn solve(input: &str) -> u32 {
        (0..25)
            .into_iter()
            .fold(parse_stones(input), |stones, _| stones.next())
            .stones
            .len() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::download_input;

    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            part1::solve("125 17"),
            55312
        );
    }

    #[test]
    fn test_result_part1() {
        assert!(
            part1::solve(&download_input(11)) > 166011
        )
    }
}
