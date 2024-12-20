use {
    std::collections::HashMap,
    num_bigint::BigUint,
    num_traits::{ops::checked::CheckedMul, FromPrimitive},
};

#[derive(Eq, PartialEq, Clone)]
struct Stone {
    number: BigUint,
}

struct Stones {
    stones: Vec<Stone>,
    stones_after_blinks: HashMap<(BigUint, u32), u64>,
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
            stones_after_blinks: HashMap::new(),
        }
    }

    fn stones_after_blinks(&mut self, blinks: u32) -> u64 {
        let mut result = 0;
        for stone in &self.stones.clone() {
            result += self.stones_after_blinks_for_stone(stone, blinks);
        }
        result
    }

    fn stones_after_blinks_for_stone(&mut self, stone: &Stone, blinks: u32) -> u64 {
        if blinks == 0 {
            return 1; // if no more blinks, then we are left with just this one stone
        }

        // if there is a memoized result, return it.
        if let Some(stones) = self.stones_after_blinks.get(&(stone.number.clone(), blinks)) {
            return *stones;
        }

        let result = if stone.number == BigUint::ZERO {
            self.stones_after_blinks_for_stone(&Stone::new(BigUint::from_u32(1).unwrap()), blinks - 1)
        } else {
            let digits = stone.number.to_radix_be(10);
            if digits.len() % 2 == 0 {
                self.stones_after_blinks_for_stone(
                    &Stone::new(BigUint::from_radix_be(&digits[0..digits.len() / 2], 10).unwrap()),
                    blinks - 1
                ).checked_add(self.stones_after_blinks_for_stone(
                    &Stone::new(BigUint::from_radix_be(&digits[digits.len()/2..], 10).unwrap()),
                    blinks - 1
                )).unwrap()
            } else {
                self.stones_after_blinks_for_stone(&Stone::new(stone.number.checked_mul(&BigUint::from_u32(2024).unwrap()).unwrap()), blinks - 1)
            }
        };

        self.stones_after_blinks.insert((stone.number.clone(), blinks), result);

        result
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

fn solve(input: &str, blinks: u32) -> u64 {
    parse_stones(input).stones_after_blinks(blinks)
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
