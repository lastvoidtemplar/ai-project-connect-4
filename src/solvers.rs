use crate::positions::{HEIGHT, Position, WIDTH};

pub const MIN_SCORE: i32 = -(WIDTH as i32 * HEIGHT as i32) / 2;
pub const MAX_SCORE: i32 = (WIDTH as i32 * HEIGHT as i32) / 2;

pub trait Solver {
    fn solve(&mut self, position: &mut Box<dyn Position>) -> i32;
    fn explored_nodes(&self) -> usize;
}

pub mod alpha_beta_solver;
pub mod negamax_solver;
