use crate::positions::{HEIGHT, WIDTH};

pub const MIN_SCORE: i32 = -(WIDTH as i32 * HEIGHT as i32) / 2;
pub const MAX_SCORE: i32 = (WIDTH as i32 * HEIGHT as i32) / 2;

pub trait Solver {
    fn solve(&mut self) -> i32;
    fn explored_nodes(&self) -> usize;
}

pub mod bitboard_solver;
pub mod center_columns_solver;
pub mod alpha_beta_solver;
pub mod negamax_solver;
