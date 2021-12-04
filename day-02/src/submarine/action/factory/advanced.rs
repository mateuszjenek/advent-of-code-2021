use crate::submarine::Position;
use crate::submarine::Action;

use crate::submarine::action::factory::Factory;

#[derive(Copy, Clone)]
pub struct Advanced {}

impl Factory for Advanced {
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
        depth: position.depth,
        aim: position.aim - value
    }
}

fn down(value: i32, position: Position) -> Position {
    return Position {
        forward: position.forward,
        depth: position.depth,
        aim: position.aim + value
    }
}

fn forward(value: i32, position: Position) -> Position {
    return Position {
        forward: position.forward + value,
        depth: position.depth + (position.aim * value),
        aim: position.aim
    }
}
