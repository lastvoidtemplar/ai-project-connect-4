use crate::positions::Position;

pub trait Solver {
    fn solve<P: Position>(&mut self, position: &mut P) -> i32;
    fn explored_nodes(&self) -> usize;
}

pub mod negamax_solver;