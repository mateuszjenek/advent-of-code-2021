pub mod factory;

use crate::submarine::position::Position;

pub struct Action {
    pub value: i32,
    pub calculate_position: fn(i32, Position) -> Position
}

impl Action {
    pub fn calculate_position(&self, position: Position) -> Position {
        return (self.calculate_position)(self.value, position)
    }
}
