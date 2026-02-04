use std::{cmp::max, i32};

use crate::{positions::{HEIGHT, Position, WIDTH}, solvers::Solver};

pub struct NegamaxSolver {
    explored_nodes: usize
}

impl NegamaxSolver {
    pub fn new() -> Self {
        Self { explored_nodes: 0 }
    }

     fn negamax(&mut self, position: &mut Box<dyn Position>) -> i32 {
        self.explored_nodes+=1;
        if position.played_moves() == WIDTH * HEIGHT {
            return 0;
        }

        for colm in 0..WIDTH{
            if position.can_play(colm) && position.is_winning(colm){
                return (WIDTH * HEIGHT - position.played_moves() + 1) as i32 / 2;
            }
        }

        let mut best_score = i32::MIN;
        for colm in 0..WIDTH {
            if position.can_play(colm) {
                position.play(colm);
                best_score = max(best_score, -self.negamax(position));
                position.reverse_play(colm);
            }
        }

        return best_score;
    }
}

impl Solver for NegamaxSolver {
    fn solve(&mut self, position: &mut Box<dyn Position>) -> i32 {
        self.explored_nodes = 0;
        self.negamax(position)
    }

    fn explored_nodes(&self) -> usize {
        self.explored_nodes
    }

}