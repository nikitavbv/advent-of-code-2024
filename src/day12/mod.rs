use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
        ]
    }
}

#[derive(Clone, Debug)]
struct Plot {
    plant_type: char,
}

impl Plot {
    fn new(plant_type: char) -> Self {
        Self {
            plant_type,
        }
    }
}

struct World {
    // (0, 0) is top left
    map: Vec<Vec<Plot>>,
}

impl World {
    fn new(map: Vec<Vec<Plot>>) -> Self {
        Self {
            map,
        }
    }

    fn at(&self, position: &Position) -> Option<&Plot> {
        if position.x < 0 || position.y < 0 {
            return None;
        }

        self.map.get(position.y as usize)
            .and_then(|row| row.get(position.x as usize))
    }

    fn regions(&self) -> Vec<Region> {
        let mut result = Vec::new();

        let mut explored_positions = HashSet::new();

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let position = Position::new(x as i32, y as i32);
                if explored_positions.contains(&position) {
                    continue;
                }

                // create a new region
                let mut region_positions = HashSet::new();
                let plot = self.at(&position).unwrap();
                let region_plant_type = plot.plant_type.clone();

                self.explore_region(region_plant_type, &position, &mut region_positions, &mut explored_positions);

                result.push(Region::new(plot.clone(), region_positions.into_iter().collect()));
            }
        }

        result
    }

    fn explore_region(&self, plant_type: char, position: &Position, region_positions: &mut HashSet<Position>, explored_positions: &mut HashSet<Position>) {
        if explored_positions.contains(position) {
            // already explored
            return;
        }

        if region_positions.contains(position) {
            // already explored
            return;
        }

        let plot = match self.at(position) {
            Some(v) => v,
            None => {
                // invalid position
                return;
            }
        };

        if plot.plant_type != plant_type {
            // this plant type does not belong to this region
            return;
        }

        region_positions.insert(position.clone());
        explored_positions.insert(position.clone());

        position.neighbors()
            .into_iter()
            .for_each(|position| self.explore_region(plant_type, &position, region_positions, explored_positions));
    }

    fn total_cost(&self) -> u32 {
        self.regions().into_iter().map(|v| v.cost()).sum()
    }
}

#[derive(Debug)]
struct Region {
    plot: Plot,
    positions: Vec<Position>,
}

impl Region {
    fn new(plot: Plot, positions: Vec<Position>) -> Self {
        Self {
            plot,
            positions,
        }
    }

    fn area(&self) -> u32 {
        self.positions.len() as u32
    }

    fn perimeter(&self) -> u32 {
        let mut total_perimiter = 0;
        let positions_set = self.positions.iter().cloned().collect::<HashSet<Position>>();

        for position in &self.positions {
            for neighbor in position.neighbors() {
                if positions_set.contains(&neighbor) {
                    continue;
                }
                total_perimiter += 1;
            }
        }

        total_perimiter
    }

    fn cost(&self) -> u32 {
        self.area().checked_mul(self.perimeter()).unwrap()
    }
}

fn parse_world(s: &str) -> World {
    World::new(s.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().into_iter().map(|c| Plot::new(c)).collect())
        .collect())
}

pub mod part1 {
    use {
        crate::utils::download_input,
        super::*,
    };

    #[allow(dead_code)]
    pub fn run() {
        println!("result: {}", parse_world(&download_input(12)).total_cost());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let world = parse_world("AAAA
BBCD
BBCC
EEEC");
        assert_eq!(world.total_cost(), 140);
    }

    #[test]
    fn example() {
        let world = parse_world("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE");
        assert_eq!(world.total_cost(), 1930);
    }
}
