use crate::submarine::Position;
use crate::submarine::Action;

use crate::submarine::action::factory::Factory;

#[derive(Copy, Clone)]
pub struct Simple {}

impl Factory for Simple {
    fn make_up(self, value: i32) -> Action { 
        return Action{
            value: value,
            calculate_position: up
        }
    }
    fn make_down(self, value: i32) -> Action { 
        return Action{
            value: value,
            calculate_position: down
        }
    }
    fn make_forward(self, value: i32) -> Action { 
        return Action{
            value: value,
            calculate_position: forward
        }
    }
}

fn up(value: i32, position: Position) -> Position {
    return Position {
        forward: position.forward,
        depth: position.depth - value,
        aim: position.aim
    }
}

fn down(value: i32, position: Position) -> Position {
    return Position {
        forward: position.forward,
        depth: position.depth + value,
        aim: position.aim
    }
}

fn forward(value: i32, position: Position) -> Position {
    return Position {
        forward: position.forward + value,
        depth: position.depth,
        aim: position.aim
    }
}
