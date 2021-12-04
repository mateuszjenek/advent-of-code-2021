use crate::submarine::Action;

mod simple;
mod advanced;

pub use self::simple::Simple;
pub use self::advanced::Advanced;

pub trait Factory {
    fn make_up(self, value: i32) -> Action;
    fn make_down(self, value: i32) -> Action;
    fn make_forward(self, value: i32) -> Action;
}
