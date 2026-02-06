use std::{cmp::max, i32};

use crate::{
    positions::{HEIGHT, Position, WIDTH, bit_position::BitPosition},
    solvers::{MIN_SCORE, Solver}, transposition_table::TranspositionTable,
};

pub struct IterativeDeepeningSolver {
    position: BitPosition,
    alpha: i32,
    beta: i32,
    table: TranspositionTable,
    explored_nodes: usize,
    column_order: [usize; WIDTH],
}

impl IterativeDeepeningSolver {
    pub fn new(position: BitPosition, alpha: i32, beta: i32, table: TranspositionTable) -> Self {
        let mut column_order = [0; WIDTH];

        // [3, 2, 4, 1, 5, 0, 6]
        for ind in 0..(WIDTH as i32) {
            let colm = WIDTH as i32 / 2 + (1 - 2 * (ind & 1)) * (ind + 1) / 2;
            column_order[ind as usize] = colm as usize;
        }

        Self {
            position,
            explored_nodes: 0,
            alpha,
            beta,
            table,
            column_order,
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

        let mut upper_bound = (WIDTH * HEIGHT - self.position.played_moves() - 1) as i32 / 2;
        if let Some(value) = self.table.get(self.position.key()) {
            upper_bound = value as i32 + MIN_SCORE  as i32 - 1;
        }
        if upper_bound < beta {
            beta = upper_bound;
            if alpha >= beta {
                return beta;
            }
        }

        for ind in 0..WIDTH {
            let colm = self.column_order[ind];
            if self.position.can_play(colm) {
                let old_position = self.position;
                self.position.play(colm);
                alpha = max(alpha, -self.negamax(-beta, -alpha));
                self.position = old_position;
                if alpha >= beta {
                    return alpha;
                }
            }
        }
        self.table.put(self.position.key(), (alpha - MIN_SCORE + 1) as u8);
        return alpha;
    }
}

impl Solver for IterativeDeepeningSolver {
    fn solve(&mut self) -> i32 {
        self.explored_nodes = 0;
        
        let mut left = self.alpha;
        let mut right = self.beta;

        while left < right {
            let mut median = left + (right - left) / 2;
            if median <= 0  &&  left/2 < median {
                median = left / 2;
            } else if median >= 0 && median < right / 2 {
                median = right / 2;
            }
            let score = self.negamax(median, median + 1);
            if score <= median {
                right = score;
            } else {
                left = score;
            }
        }

        left
    }

    fn explored_nodes(&self) -> usize {
        self.explored_nodes
    }
}