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
            } else {
                let digits = stone.number.to_radix_le(10);
                if digits.len() % 2 == 0 {
                    result.push(Stone::new(BigUint::from_radix_le(&digits[0..digits.len()/2], 10).unwrap()));
                    result.push(Stone::new(BigUint::from_radix_le(&digits[digits.len()/2..], 10).unwrap()));
                } else {
                    result.push(Stone::new(stone.number.checked_mul(&BigUint::from_u32(2024).unwrap()).unwrap()));
                }
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

fn solve(input: &str, blinks: u32) -> u32 {
    (0..blinks)
        .into_iter()
        .fold(parse_stones(input), |stones, i| {
            println!("{}", i);
            stones.next()
        })
        .stones
        .len() as u32
}

pub mod part1 {
    use {
        crate::utils::download_input,
        super::solve,
    };

    #[allow(dead_code)]
    pub fn run() {
        println!("result: {}", solve(&download_input(11), 25));
    }
}

pub mod part2 {
    use {
        crate::utils::download_input,
        super::solve,
    };

    #[allow(dead_code)]
    pub fn run() {
        println!("result: {}", solve(&download_input(11), 75));
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::download_input;

    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            solve("125 17", 25),
            55312
        );
    }

    #[test]
    fn test_result_part1() {
        assert!(
            solve(&download_input(11), 25) > 166011
        )
    }
}
