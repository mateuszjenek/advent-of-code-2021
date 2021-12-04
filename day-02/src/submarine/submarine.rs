use crate::submarine::action::Action;
use crate::submarine::position::Position;

pub struct Submarine {
    pub position: Position
}

impl Submarine {
    pub fn play(&mut self, action: Action) {
        self.position = action.calculate_position(self.position);
    }
}