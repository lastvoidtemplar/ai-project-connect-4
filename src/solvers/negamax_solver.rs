use std::{cmp::max, i32};

use crate::{positions::{HEIGHT, Position, WIDTH, array_position::ArrayPosition}, solvers::Solver};

pub struct NegamaxSolver {
    position: ArrayPosition,
    explored_nodes: usize
}

impl NegamaxSolver {
    pub fn new(position: ArrayPosition) -> Self {
        Self {position , explored_nodes: 0 }
    }

     fn negamax(&mut self) -> i32 {
        self.explored_nodes+=1;
        if self.position.played_moves() == WIDTH * HEIGHT {
            return 0;
        }

        for colm in 0..WIDTH{
            if self.position.can_play(colm) && self.position.is_winning(colm){
                return (WIDTH * HEIGHT - self.position.played_moves() + 1) as i32 / 2;
            }
        }

        let mut best_score = i32::MIN;
        for colm in 0..WIDTH {
            if self.position.can_play(colm) {
                self.position.play(colm);
                best_score = max(best_score, -self.negamax());
                self.position.reverse_play(colm);
            }
        }

        return best_score;
    }
}

impl Solver for NegamaxSolver {
    fn solve(&mut self) -> i32 {
        self.explored_nodes = 0;
        self.negamax()
    }

    fn explored_nodes(&self) -> usize {
        self.explored_nodes
    }

}