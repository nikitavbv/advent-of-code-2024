use std::collections::{HashMap, HashSet};

const MAX_HEIGHT: u8 = 9;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn neighbours(&self) -> Vec<Position> {
        vec![
            if self.y > 0 {
                Some(Position {
                    x: self.x,
                    y: self.y - 1,
                })
            } else {
                None
            },
            Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
            Some(Position {
                x: self.x,
                y: self.y + 1,
            }),
            if self.x > 0 {
                Some(Position {
                    x: self.x - 1,
                    y: self.y,
                })
            } else {
                None
            }
        ].into_iter().filter_map(|v| v).collect()
    }
}

pub struct Map {
    // top left is (0, 0).
    map: Vec<Vec<u8>>,
    reachable_heights: HashMap<Position, HashSet<Position>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            reachable_heights: HashMap::new(),
        }
    }

    fn from_map_vec(map: Vec<Vec<u8>>) -> Self {
        Self {
            map,
            reachable_heights: HashMap::new(),
        }
    }

    pub fn at(&self, position: &Position) -> Option<u8> {
        self.map.get(position.x as usize).and_then(|v| v.get(position.y as usize)).map(|v| *v)
    }

    pub fn reachable_heights_from_position(&mut self, position: &Position, skip_positions: HashSet<Position>) -> HashSet<Position> {
        if skip_positions.contains(position) {
            return HashSet::new(); // if already visited position, return empty set.
        }

        let self_height = match self.at(position) {
            Some(v) => v,
            None => return HashSet::new(), // if invalid position, return empty set
        };

        // if this position is top, return itself
        if self_height == MAX_HEIGHT {
            let mut self_set = HashSet::new();
            self_set.insert(position.clone());
            return self_set;
        }

        // if have positions memoized, return them
        if let Some(heights) = self.reachable_heights.get(position) {
            return heights.clone();
        }

        // else, let's compute based on neighbours
        let mut visited = skip_positions.clone();
        visited.insert(position.clone());

        let mut result = HashSet::new();
        for other in position.neighbours() {
            let other_height = match self.at(&other) {
                Some(v) => v,
                None => continue,
            };

            if other_height != self_height + 1 {
                continue;
            }

            for height in self.reachable_heights_from_position(&other, visited.clone()) {
                result.insert(height);
            }
        }

        self.reachable_heights.insert(position.clone(), result.clone());

        result
    }
}

fn parse_map(input: &str) -> Map {
    Map::from_map_vec(input.lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().into_iter().filter_map(|v| v.to_digit(10).map(|v| v as u8)).collect())
        .collect())
}

fn solve(input: &str) -> u32 {
    let mut map = parse_map(input);
    let mut total = 0;
    for y in 0..map.map.len() {
        for x in 0..map.map[y].len() {
            let position = Position::new(x as u32, y as u32);
            if map.at(&position).unwrap() == 0 {
                total += map.reachable_heights_from_position(&position, HashSet::new()).len() as u32;
            }
        }
    }

    total
}

pub mod part1 {
    use {
        crate::utils::download_input,
        super::*,
    };

    pub fn run() {
        println!("result: {}", solve(&download_input(10)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            solve(r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#),
            36
        );
    }
}
