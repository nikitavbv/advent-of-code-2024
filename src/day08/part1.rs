use {
    rpds::{HashTrieMapSync, HashTrieSet, HashTrieSetSync},
    crate::utils::download_input,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    // top left corner is (0, 0)
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

    fn diff(&self, other: &Position) -> (i32, i32) {
        (
            self.x as i32 - other.x as i32,
            self.y as i32 - other.y as i32
        )
    }
}

#[derive(Debug)]
struct Map {
    antennas: HashTrieMapSync<char, HashTrieSetSync<Position>>,
    rows: u32,
    columns: u32,
}

impl Map {
    pub fn new(rows: u32, columns: u32) -> Self {
        Self {
            antennas: HashTrieMapSync::new_sync(),
            rows,
            columns,
        }
    }

    pub fn add_antenna(self, frequency: char, position: Position) -> Self {
        let antennas = self.antennas.insert(
            frequency,
            self.antennas.get(&frequency).cloned().unwrap_or(HashTrieSetSync::new_sync())
                .insert(position)
        );

        Self {
            antennas,
            ..self
        }
    }
}

pub fn parse_map(s: &str) -> Map {
    let lines = s.lines().filter(|line| !line.is_empty()).collect::<Vec<_>>();

    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter(|(_x, frequency)| frequency.is_ascii_digit() || frequency.is_ascii_alphabetic())
            .map(move |(x, frequency)| (frequency, Position::new(x as u32, y as u32)))
        )
        .fold(
            Map::new(lines.len() as u32, lines[0].len() as u32),
            |map, (frequency, position)| map.add_antenna(frequency, position)
        )
}

fn is_antinode_position_for_antennas(position: &Position, antennas: &HashTrieSetSync<Position>) -> bool {
    let distances = antennas
        .iter()
        .map(|antenna_position| position.diff(antenna_position))
        .filter(|distance| distance.0 != 0 || distance.1 != 0)
        .collect::<HashTrieSet<_>>();

    distances
        .iter()
        .find(|distance| distances.contains(&(distance.0 * 2, distance.1 * 2)))
        .is_some()
}

fn is_antinode_position(position: &Position, map: &Map) -> bool {
    map.antennas
        .iter()
        .find(|(_frequency, antennas)| is_antinode_position_for_antennas(position, antennas))
        .is_some()
}

fn solve(map: &Map) -> u32 {
    (0..map.rows)
        .into_iter()
        .flat_map(|y| (0..map.columns).into_iter().map(move |x| Position::new(x, y)))
        .filter(|position| is_antinode_position(position, map))
        .count() as u32
}

#[allow(dead_code)]
pub fn run() {
    println!("result: {}", solve(&parse_map(&download_input(8))));
}

#[cfg(test)]
mod tests {
    use {
        lazy_static::lazy_static,
        super::*,
    };

    lazy_static! {
        static ref EXAMPLE_MAP: Map = parse_map(r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#);

        static ref EXAMPLE_MAP_1: Map = parse_map(r#"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."#);
    }

    #[test]
    fn example() {
        assert_eq!(solve(&EXAMPLE_MAP), 14);
    }

    #[test]
    fn example_1() {
        assert_eq!(solve(&EXAMPLE_MAP_1), 2);
    }
}
