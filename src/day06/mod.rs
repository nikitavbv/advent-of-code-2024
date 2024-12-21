use std::collections::HashSet;

pub mod part1;
pub mod part2;

#[derive(Clone)]
enum Object {
    Empty,
    Obstacle,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn move_in_direction(&self, direction: &Direction) -> Option<Self> {
        Some(match direction {
            Direction::Top => if self.y != 0 {
                Self::new(self.x, self.y - 1)
            } else {
                return None;
            },
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Bottom => Self::new(self.x, self.y + 1),
            Direction::Left => if self.x != 0 {
                Self::new(self.x - 1, self.y)
            } else {
                return None;
            },
        })
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
            Self::Left => Self::Top,
        }
    }

    #[allow(dead_code)]
    pub fn is_vertical(&self) -> bool {
        match self {
            Self::Top | Self::Bottom => true,
            Self::Left | Self::Right => false,
        }
    }
}

#[derive(Clone)]
pub struct World {
    // top left is (0, 0)
    map: Vec<Vec<Object>>,
    guard_position: Option<Position>,
    guard_direction: Option<Direction>,
}

impl World {
    fn new(map: Vec<Vec<Object>>, guard_position: Position, guard_direction: Direction) -> Self {
        Self {
            map,
            guard_position: Some(guard_position),
            guard_direction: Some(guard_direction),
        }
    }

    fn object_at(&self, position: &Position) -> Option<&Object> {
        self.map.get(position.y as usize).and_then(|row| row.get(position.x as usize))
    }

    pub fn with_obstacle_at(&self, position: &Position) -> Self {
        let mut map = self.map.clone();
        map[position.y as usize][position.x as usize] = Object::Obstacle;

        Self {
            map,
            guard_position: self.guard_position.clone(),
            guard_direction: self.guard_direction.clone(),
        }
    }

    pub fn next(self) -> Self {
        let guard_position = match &self.guard_position {
            Some(v) => v,
            None => return self,
        };
        let guard_direction = match &self.guard_direction {
            Some(v) => v,
            None => return self,
        };

        let next_position = match guard_position.move_in_direction(guard_direction) {
            Some(v) => v,
            None => {
                // guard just exited the map
                return Self {
                    map: self.map,
                    guard_position: None,
                    guard_direction: None,
                };
            }
        };
        let object_at_next_position = match self.object_at(&next_position) {
            Some(v) => v,
            None => {
                // guard just exited the map
                return Self {
                    map: self.map,
                    guard_position: None,
                    guard_direction: None,
                };
            }
        };

        let (guard_position, guard_direction) = match object_at_next_position {
            Object::Empty => (next_position, guard_direction.clone()), // continue moving in same direction
            Object::Obstacle => (guard_position.clone(), guard_direction.turn_right()), // turn right because there is an obstacle in front of guard
        };

        Self {
            map: self.map,
            guard_position: Some(guard_position),
            guard_direction: Some(guard_direction),
        }
    }

    pub fn encode_to_string(&self) -> String {
        self.map.iter()
            .map(|row| row.iter().map(|obj| match obj {
                Object::Empty => '.',
                Object::Obstacle => '#',
            }).collect::<String>())
            .collect::<Vec<_>>().join("\n")
    }
}

pub struct VisitedPositions {
    pub visited_positions: HashSet<Position>,
    pub loops: bool,
}

impl VisitedPositions {
    pub fn total_positions(&self) -> u32 {
        self.visited_positions.len() as u32
    }
}

pub fn calculate_visited_positions(mut world: World) -> VisitedPositions {
    let mut visited_positions = HashSet::new();
    let mut visited_positions_with_directions = HashSet::new();

    loop {
        visited_positions.insert(world.guard_position.clone().unwrap());
        if !visited_positions_with_directions.insert((
            world.guard_position.clone().unwrap(),
            world.guard_direction.clone().unwrap(),
        )) {
            return VisitedPositions {
                visited_positions,
                loops: true,
            };
        }

        world = world.next();
        if world.guard_position.is_none() {
            break;
        }
    }

    VisitedPositions {
        visited_positions,
        loops: false,
    }
}

pub fn parse_world(s: &str) -> World {
    let lines = s.lines().filter(|v| !v.is_empty()).collect::<Vec<_>>();

    let mut map = Vec::new();
    let mut guard_position = None;
    let mut guard_direction = None;

    for y in 0..lines.len() {
        let mut row = Vec::new();
        for x in 0..lines[0].len() {
            let position = Position::new(x as u32, y as u32);

            let object = match lines[y].chars().nth(x).unwrap() {
                '#' => Object::Obstacle,
                '^' => {
                    guard_position = Some(position);
                    guard_direction = Some(Direction::Top);
                    Object::Empty
                },
                '>' => {
                    guard_position = Some(position);
                    guard_direction = Some(Direction::Right);
                    Object::Empty
                },
                'v' => {
                    guard_position = Some(position);
                    guard_direction = Some(Direction::Bottom);
                    Object::Empty
                },
                '<' => {
                    guard_position = Some(position);
                    guard_direction = Some(Direction::Left);
                    Object::Empty
                },
                _ => Object::Empty
            };
            row.push(object);
        }
        map.push(row);
    }

    World::new(
        map,
        guard_position.expect("world without guard"),
        guard_direction.expect("world without guard")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let world = parse_world(r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#);
        assert_eq!(calculate_visited_positions(world).total_positions(), 41);
    }

    #[test]
    fn test_detect_loop() {
        let world = parse_world(r#"....#.....
....+---+#
....|...|.
..#.|...|.
....|..#|.
....|...|.
.#.#^---+.
........#.
#.........
......#..."#);
        assert_eq!(calculate_visited_positions(world).loops, true);
    }
}
