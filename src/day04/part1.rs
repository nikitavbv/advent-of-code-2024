use crate::utils::download_input;

type Direction = [[isize; 2]; 4];

const HORIZONTAL: Direction = [
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
const HORIZONTAL_1: [[isize; 2]; 4] = [
    [0, 0],
    [-1, 1],
    [-2, 2],
    [-3, 3],
];
const HORIZONTAL_2: [[isize; 2]; 4] = [
    [0, 0],
    [1, 1],
    [2, 2],
    [3, 3],
];
const HORIZONTAL_3: [[isize; 2]; 4] = [
    [0, 0],
    [1, -1],
    [2, -2],
    [3, -3],
];
const HORIZONTAL_4: [[isize; 2]; 4] = [
    [0, 0],
    [-1, -1],
    [-2, -2],
    [-3, -3],
];

const ALL_DIRECTIONS: [Direction; 8] = [
    HORIZONTAL,
    HORIZONTAL_REVERSED,
    VERTICAL,
    VERTICAL_REVERSED,
    HORIZONTAL_1,
    HORIZONTAL_2,
    HORIZONTAL_3,
    HORIZONTAL_4,
];

const WORD: &str = "XMAS";

struct Grid {
    // (0, 0) is top left corner
    grid: Vec<String>,
}

impl Grid {
    pub fn new(grid: String) -> Self {
        Self {
            grid: grid.lines().filter(|v| !v.is_empty()).map(|v| v.to_owned()).collect(),
        }
    }

    fn rows(&self) -> usize {
        self.grid.len()
    }

    fn columns(&self) -> usize {
        self.grid[0].len()
    }

    fn at(&self, row: usize, column: usize) -> Option<char> {
        self.grid.get(row)?.chars().nth(column)
    }

    fn select_word(&self, row: usize, column: usize, direction: &Direction) -> Option<String> {
        let positions = direction
            .iter()
            .map(|[d_row, d_column]| (row as isize + d_row, column as isize + d_column))
            .map(|(row, column)| {
                if row < 0 {
                    None
                } else if column < 0 {
                    None
                } else {
                    self.at(row as usize, column as usize)
                }
            })
            .collect::<Vec<_>>();

        if positions.iter().find(|v| v.is_none()).is_some() {
            return None;
        }

        Some(positions.into_iter().map(|v| v.unwrap()).collect())
    }
}

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
    ALL_DIRECTIONS.iter()
        .map(|direction| grid.select_word(row, column, direction))
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
