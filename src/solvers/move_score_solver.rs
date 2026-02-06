use std::{cmp::max, i32};

use crate::{
    move_sorter::MoveSorter,
    positions::{
        HEIGHT, WIDTH,
        advance_bit_position::{AdvanceBitPosition, column_mask},
    },
    solvers::{MIN_SCORE, Solver},
    transposition_table::TranspositionTable,
};

pub struct MoveScoreSolver {
    position: AdvanceBitPosition,
    alpha: i32,
    beta: i32,
    table: TranspositionTable,
    explored_nodes: usize,
    column_order: [usize; WIDTH],
}

impl MoveScoreSolver {
    pub fn new(
        position: AdvanceBitPosition,
        alpha: i32,
        beta: i32,
        table: TranspositionTable,
    ) -> Self {
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

        let next = self.position.possible_non_losing_moves();
        if next == 0 {
            return -(((WIDTH * HEIGHT - self.position.played_moves()) / 2) as i32);
        }

        if self.position.played_moves() >= WIDTH * HEIGHT - 2 {
            // draw
            return 0;
        }

        // opponent cannot win with his next move
        let lower_bound = -(((WIDTH * HEIGHT - self.position.played_moves() - 2) / 2) as i32);
        if alpha < lower_bound {
            alpha = lower_bound;
            if alpha >= beta {
                return alpha;
            }
        }

        let mut upper_bound = (WIDTH * HEIGHT - self.position.played_moves() - 1) as i32 / 2;
        if let Some(value) = self.table.get(self.position.key()) {
            upper_bound = value as i32 + MIN_SCORE as i32 - 1;
        }
        if upper_bound < beta {
            beta = upper_bound;
            if alpha >= beta {
                return beta;
            }
        }

        let mut moves = MoveSorter::new();
        for ind in (0..WIDTH).rev() {
            let colm = self.column_order[ind];
            let mov = next & column_mask(colm);
            if mov != 0 {
                moves.add(mov, self.position.score(mov));
            }
        }

        for mov in moves {
            let old_position = self.position;
            self.position.play_move(mov);
            alpha = max(alpha, -self.negamax(-beta, -alpha));
            self.position = old_position;
            if alpha >= beta {
                return alpha;
            }
        }
        self.table
            .put(self.position.key(), (alpha - MIN_SCORE + 1) as u8);
        return alpha;
    }
}

impl Solver for MoveScoreSolver {
    fn solve(&mut self) -> i32 {
        self.explored_nodes = 0;

        if self.position.can_win_next() {
            return (WIDTH * HEIGHT - self.position.played_moves() + 1) as i32 / 2;
        }

        let mut left = self.alpha;
        let mut right = self.beta;

        while left < right {
            let mut median = left + (right - left) / 2;
            if median <= 0 && left / 2 < median {
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
