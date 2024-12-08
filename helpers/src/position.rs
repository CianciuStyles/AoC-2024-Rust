use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Position {
        Position { row, col }
    }

    pub fn from_usize(row: usize, col: usize) -> Position {
        Position {
            row: i32::try_from(row).expect("Invalid value for row"),
            col: i32::try_from(col).expect("Invalid value for col"),
        }
    }

    pub fn get_row(self) -> i32 {
        self.row
    }
    
    pub fn get_col(self) -> i32 {
        self.col
    }

    pub fn is_within_bounds(self, max_rows: usize, max_cols: usize) -> bool {
        self.row >= 0
            && self.row < i32::try_from(max_rows).expect("Invalid value for max_rows")
            && self.col >= 0
            && self.col < i32::try_from(max_cols).expect("Invalid value for max_cols")
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.row - rhs.row, self.col - rhs.col)
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}
