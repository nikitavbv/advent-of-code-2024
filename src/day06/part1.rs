use {
    std::collections::HashSet,
    crate::utils::download_input,
};

enum Object {
    Empty,
    Obstacle,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Position {
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

    fn move_in_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Top => Self::new(self.x, self.y - 1),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Bottom => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Clone)]
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
}

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

    pub fn next(self) -> Self {
        let guard_position = match &self.guard_position {
            Some(v) => v,
            None => return self,
        };
        let guard_direction = match &self.guard_direction {
            Some(v) => v,
            None => return self,
        };

        let next_position = guard_position.move_in_direction(guard_direction);
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
        let guard_direction: Direction = match object_at_next_position {
            Object::Empty => guard_direction.clone(), // continue moving in same direction
            Object::Obstacle => guard_direction.turn_right(), // turn right because there is an obstacle in front of guard
        };
        let guard_position = guard_position.move_in_direction(&guard_direction);

        Self {
            map: self.map,
            guard_position: Some(guard_position),
            guard_direction: Some(guard_direction),
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("result: {}", solve(parse_world(&download_input(6))));
}

fn solve(mut world: World) -> u32 {
    let mut visited_positions = HashSet::new();

    loop {
        visited_positions.insert(world.guard_position.clone().unwrap());
        world = world.next();
        if world.guard_position.is_none() {
            break;
        }
    }

    visited_positions.len() as u32
}

fn parse_world(s: &str) -> World {
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
        assert_eq!(solve(world), 41);
    }
}
