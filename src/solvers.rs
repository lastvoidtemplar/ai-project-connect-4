use crate::positions::{HEIGHT, WIDTH};

// it needs at least 4 moves to win
pub const MIN_SCORE: i32 = -(WIDTH as i32 * HEIGHT as i32) / 2 + 3;
pub const MAX_SCORE: i32 = (WIDTH as i32 * HEIGHT as i32 + 1) / 2 - 3;

pub trait Solver {
    fn solve(&mut self) -> i32;
    fn explored_nodes(&self) -> usize;
}

pub mod move_score_solver;
pub mod avoid_losing_moves_solver;
pub mod iterative_deepening_solver;
pub mod transposition_table_solver;
pub mod bitboard_solver;
pub mod center_columns_solver;
pub mod alpha_beta_solver;
pub mod negamax_solver;
