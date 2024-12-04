use {
    crate::utils::download_input,
    super::{Grid, Direction4},
};

const HORIZONTAL: Direction4 = [
    [0, 0],
    [0, 1],
    [0, 2],
    [0, 3],
];
const HORIZONTAL_REVERSED: [[isize; 2]; 4] = [
    [0, 0],
    [0, -1],
    [0, -2],
    [0, -3],
];
const VERTICAL: [[isize; 2]; 4] = [
    [0, 0],
    [1, 0],
    [2, 0],
    [3, 0],
];
const VERTICAL_REVERSED: [[isize; 2]; 4] = [
    [0, 0],
    [-1, 0],
    [-2, 0],
    [-3, 0],
];
// think of it like clock hand
const DIAGONAL_1: [[isize; 2]; 4] = [
    [0, 0],
    [-1, 1],
    [-2, 2],
    [-3, 3],
];
const DIAGONAL_2: [[isize; 2]; 4] = [
    [0, 0],
    [1, 1],
    [2, 2],
    [3, 3],
];
const DIAGONAL_3: [[isize; 2]; 4] = [
    [0, 0],
    [1, -1],
    [2, -2],
    [3, -3],
];
const DIAGONAL_4: [[isize; 2]; 4] = [
    [0, 0],
    [-1, -1],
    [-2, -2],
    [-3, -3],
];

const ALL_DIRECTIONS4: [Direction4; 8] = [
    HORIZONTAL,
    HORIZONTAL_REVERSED,
    VERTICAL,
    VERTICAL_REVERSED,
    DIAGONAL_1,
    DIAGONAL_2,
    DIAGONAL_3,
    DIAGONAL_4,
];

const WORD: &str = "XMAS";

#[allow(dead_code)]
pub fn run() {
    println!("result: {}", solve(Grid::new(download_input(4))));
}

fn solve(grid: Grid) -> u32 {
    let mut total = 0;
    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            total += solve_at_position(&grid, row, column);
        }
    }
    total
}

fn solve_at_position(grid: &Grid, row: usize, column: usize) -> u32 {
    if !grid.at(row, column).map(|v| v == 'X').unwrap_or(false) {
        return 0;
    }
    ALL_DIRECTIONS4.iter()
        .map(|direction| grid.select_word4(row, column, direction))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .filter(|v| v == WORD)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            solve(
                Grid::new(r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#.to_owned())
            ),
            18
        );
    }
}
