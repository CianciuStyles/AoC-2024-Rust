use crate::direction::Direction;
use crate::position::Position;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct PositionWithDirection {
    position: Position,
    direction: Direction,
}

impl PositionWithDirection {
    pub fn new(row: i32, col: i32, direction: Direction) -> Self {
        PositionWithDirection {
            position: Position::new(row, col),
            direction,
        }
    }

    pub fn from_position_and_direction(position: Position, direction: Direction) -> Self {
        PositionWithDirection {
            position,
            direction,
        }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }
}
