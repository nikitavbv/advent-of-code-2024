pub mod part1;
pub mod part2;

type Direction3 = [[isize; 2]; 3];
type Direction4 = [[isize; 2]; 4];

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

    fn select_word3(&self, row: usize, column: usize, direction: &Direction3) -> Option<String> {
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

    fn select_word4(&self, row: usize, column: usize, direction: &Direction4) -> Option<String> {
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
