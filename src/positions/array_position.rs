use crate::positions::{HEIGHT, Position, WIDTH};

pub struct ArrayPosition {
    // 0 - empty cell, 1 - player 1, 2 - player 2
    board: [[i32; HEIGHT]; WIDTH],
    columns_heights: [usize; WIDTH],
    played_moves: usize,
}

impl ArrayPosition {
    pub fn new() -> Self {
        Self {
            board: [[0; HEIGHT]; WIDTH],
            columns_heights: [0; WIDTH],
            played_moves: 0,
        }
    }

    fn current_player(&self) -> i32 {
        (1 + (self.played_moves & 1)) as i32
    }

}

impl Position for ArrayPosition {
    fn can_play(&self, colm: usize) -> bool {
        self.columns_heights[colm] < HEIGHT
    }

    fn play(&mut self, colm: usize) {
        self.board[colm][self.columns_heights[colm]] += self.current_player();
        self.columns_heights[colm] += 1;
        self.played_moves += 1;
    }

    fn reverse_play(&mut self, colm: usize) {
        self.played_moves -= 1;
        self.columns_heights[colm] -= 1;
        self.board[colm][self.columns_heights[colm]] -= self.current_player();
    }

    fn is_winning(&self, colm: usize) -> bool {
        let current_player = self.current_player();

        let column_height = self.columns_heights[colm];
        let board_column = self.board[colm];
        if column_height >= 3
            && board_column[column_height - 1] == current_player
            && board_column[column_height - 2] == current_player
            && board_column[column_height - 3] == current_player
        {
            return true;
        }

        // -1 - / diagonal, 0 - _ horizotal, 1 - \
        for direction_y in -1..=1 {
            let mut encounter_cells_current_player = 0;
            for direction_x in (-1..=1).step_by(2) {
                let mut x = colm as i32 + direction_x;
                let mut y = column_height as i32 + direction_y * direction_x;
                while x >= 0
                    && x < WIDTH as i32
                    && y >= 0
                    && y < HEIGHT as i32
                    && self.board[x as usize][y as usize] == current_player
                {
                    encounter_cells_current_player += 1;
                    x += direction_x;
                    y += direction_y * direction_x
                }
            }
            if encounter_cells_current_player >= 3 {
                return true;
            }
        }
        return false;
    }

    fn played_moves(&self) -> usize {
        self.played_moves
    }
}
