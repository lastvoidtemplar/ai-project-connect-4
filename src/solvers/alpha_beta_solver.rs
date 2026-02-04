use std::{cmp::max, i32};

use crate::{
    positions::{HEIGHT, Position, WIDTH},
    solvers::Solver,
};

pub struct AlphaBetaSolver {
    alpha: i32,
    beta: i32,
    explored_nodes: usize,
}

impl AlphaBetaSolver {
    pub fn new(alpha: i32, beta: i32) -> Self {
        Self {
            explored_nodes: 0,
            alpha,
            beta,
        }
    }

    fn negamax(
        &mut self,
        position: &mut Box<dyn Position>,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        self.explored_nodes += 1;
        if position.played_moves() == WIDTH * HEIGHT {
            return 0;
        }

        for colm in 0..WIDTH {
            if position.can_play(colm) && position.is_winning(colm) {
                return (WIDTH * HEIGHT - position.played_moves() + 1) as i32 / 2;
            }
        }

        let upper_bound = (WIDTH * HEIGHT - position.played_moves() - 1) as i32 / 2;
        if upper_bound < beta {
            beta = upper_bound;
            if alpha >= beta {
                return beta;
            }
        }

        for colm in 0..WIDTH {
            if position.can_play(colm) {
                position.play(colm);
                alpha = max(alpha, -self.negamax(position, -beta, -alpha));
                position.reverse_play(colm);
                if alpha >= beta {
                    return alpha;
                }
            }
        }
        return alpha;
    }
}

impl Solver for AlphaBetaSolver {
    fn solve(&mut self, position: &mut Box<dyn Position>) -> i32 {
        self.explored_nodes = 0;
        self.negamax(position, self.alpha, self.beta)
    }

    fn explored_nodes(&self) -> usize {
        self.explored_nodes
    }
}
