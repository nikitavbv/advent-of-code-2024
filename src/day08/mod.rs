use rpds::{HashTrieMapSync, HashTrieSet, HashTrieSetSync};

pub mod part1;
pub mod part2;

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

fn is_antinode_position_for_antennas(position: &Position, antennas: &HashTrieSetSync<Position>, any_distance: bool) -> bool {
    let distances = antennas
        .iter()
        .map(|antenna_position| position.diff(antenna_position))
        .filter(|distance| distance.0 != 0 || distance.1 != 0)
        .collect::<HashTrieSet<_>>();

    if !any_distance {
        // part 1
        distances
            .iter()
            .find(|distance| distances.contains(&(distance.0 * 2, distance.1 * 2)))
            .is_some()
    } else {
        // part 2
        let antennas = antennas.iter().collect::<Vec<_>>();

        antennas
            .iter()
            .find(|antenna| {
                antennas.iter()
                    .filter(|other| other != antenna)
                    .find(|other| {
                        let a = antenna.y as i32 - other.y as i32;
                        let b = other.x as i32 - antenna.x as i32;
                        let c = (antenna.x as i32) * (other.y as i32) - (other.x as i32) * (antenna.y as i32);

                        a * position.x as i32 + b * position.y as i32 + c == 0
                    }).is_some()
            }).is_some()
    }
}

fn is_antinode_position(position: &Position, map: &Map, any_distance: bool) -> bool {
    map.antennas
        .iter()
        .find(|(_frequency, antennas)| is_antinode_position_for_antennas(position, antennas, any_distance))
        .is_some()
}

fn solve(map: &Map, any_distance: bool) -> u32 {
    (0..map.rows)
        .into_iter()
        .flat_map(|y| (0..map.columns).into_iter().map(move |x| Position::new(x, y)))
        .filter(|position| is_antinode_position(position, map, any_distance))
        .count() as u32
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

        static ref EXAMPLE_MAP_PART2: Map = parse_map(r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."#);
    }

    #[test]
    fn example() {
        assert_eq!(solve(&EXAMPLE_MAP, false), 14);
    }

    #[test]
    fn example_1() {
        assert_eq!(solve(&EXAMPLE_MAP_1, false), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(&EXAMPLE_MAP_PART2, true), 9);
    }
}
