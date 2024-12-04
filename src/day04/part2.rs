use {
    crate::utils::download_input,
    super::{Grid, Direction3},
};

// think of it like clock hand
const DIAGONAL_1: Direction3 = [
    [1, -1],
    [0, 0],
    [-1, 1],
];
const DIAGONAL_2: Direction3 = [
    [-1, -1],
    [0, 0],
    [1, 1],
];

const WORD: &str = "MAS";
const WORD_REVERSED: &str = "SAM";

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
    if !grid.at(row, column).map(|v| v == 'A').unwrap_or(false) {
        return 0;
    }

    match grid.select_word3(row, column, &DIAGONAL_1) {
        None => return 0,
        Some(word) => if word == WORD || word == WORD_REVERSED {
            // ok
        } else {
            return 0
        }
    };
    match grid.select_word3(row, column, &DIAGONAL_2) {
        None => return 0,
        Some(word) => if word == WORD || word == WORD_REVERSED {
            // ok
        } else {
            return 0
        }
    };

    1
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
            9
        );
    }
}
