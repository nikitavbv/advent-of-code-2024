use {
    crate::utils::download_input,
    super::{parse_world, World, calculate_visited_positions},
};

#[cfg(not(test))]
use indicatif::ProgressIterator;

#[allow(dead_code)]
pub fn run() {
    let world = parse_world(&download_input(6));
    let solution = solve(world);
    if solution <= 1605 {
        eprintln!("result ({}) is too low.", solution);
    } else if solution >= 2000 {
        eprintln!("result ({}) is too high.", solution);
    }

    println!("result: {}", solution);
}

fn solve(world: World) -> u32 {
    let mut current_path = calculate_visited_positions(world.clone()).visited_positions;
    current_path.remove(world.guard_position.as_ref().unwrap());

    let current_path = current_path.into_iter().collect::<Vec<_>>();

    let mut total_positions = 0;

    #[cfg(not(test))]
    let iter = current_path.iter().progress();
    #[cfg(test)]
    let iter = current_path.iter();

    for position in iter {
        let world = world.with_obstacle_at(position);
        if calculate_visited_positions(world).loops {
            total_positions += 1;
        }
    }

    total_positions
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
        assert_eq!(solve(world), 6);
    }
}
