use std::{cmp::max, i32};

use crate::{
    positions::{HEIGHT, Position, WIDTH, array_position::ArrayPosition},
    solvers::Solver,
};

pub struct AlphaBetaSolver {
    position: ArrayPosition,
    alpha: i32,
    beta: i32,
    explored_nodes: usize,
}

impl AlphaBetaSolver {
    pub fn new(position: ArrayPosition, alpha: i32, beta: i32) -> Self {
        Self {
            position,
            explored_nodes: 0,
            alpha,
            beta,
        }
    }

    fn negamax(&mut self, mut alpha: i32, mut beta: i32) -> i32 {
        self.explored_nodes += 1;
        if self.position.played_moves() == WIDTH * HEIGHT {
            return 0;
        }

        for colm in 0..WIDTH {
            if self.position.can_play(colm) && self.position.is_winning(colm) {
                return (WIDTH * HEIGHT - self.position.played_moves() + 1) as i32 / 2;
            }
        }

        let upper_bound = (WIDTH * HEIGHT - self.position.played_moves() - 1) as i32 / 2;
        if upper_bound < beta {
            beta = upper_bound;
            if alpha >= beta {
                return beta;
            }
        }

        for colm in 0..WIDTH {
            if self.position.can_play(colm) {
                self.position.play(colm);
                alpha = max(alpha, -self.negamax(-beta, -alpha));
                self.position.reverse_play(colm);
                if alpha >= beta {
                    return alpha;
                }
            }
        }
        return alpha;
    }
}

impl Solver for AlphaBetaSolver {
    fn solve(&mut self) -> i32 {
        self.explored_nodes = 0;
        self.negamax(self.alpha, self.beta)
    }

    fn explored_nodes(&self) -> usize {
        self.explored_nodes
    }
}
