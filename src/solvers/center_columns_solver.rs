use std::{cmp::max, i32};

use crate::{
    positions::{HEIGHT, Position, WIDTH},
    solvers::Solver,
};

pub struct CenterColumnsSolver {
    alpha: i32,
    beta: i32,
    explored_nodes: usize,
    column_order: [usize; WIDTH],
}

impl CenterColumnsSolver {
    pub fn new(alpha: i32, beta: i32) -> Self {
        let mut column_order = [0; WIDTH];

        // [3, 2, 4, 1, 5, 0, 6]
        for ind in 0..(WIDTH as i32) {
            let colm = WIDTH as i32 / 2 + (1 - 2 * (ind & 1)) * (ind + 1) / 2;
            column_order[ind as usize] = colm as usize;
        }

        Self {
            explored_nodes: 0,
            alpha,
            beta,
            column_order,
        }
    }

    fn negamax(&mut self, position: &mut Box<dyn Position>, mut alpha: i32, mut beta: i32) -> i32 {
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

        for ind in 0..WIDTH {
            let colm = self.column_order[ind];
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

impl Solver for CenterColumnsSolver {
    fn solve(&mut self, position: &mut Box<dyn Position>) -> i32 {
        self.explored_nodes = 0;
        self.negamax(position, self.alpha, self.beta)
    }

    fn explored_nodes(&self) -> usize {
        self.explored_nodes
    }
}
