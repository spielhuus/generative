use disjoint::DisjointSet;
use rand::prelude::*;

use crate::maze::{Board, Generator, State};
pub const BOOL_TRUE_PROBABILITY: f64 = 0.5;

enum IState {
    Start,
    Merge,
    Bottom,
    Last,
    Finished,
}

pub struct Eller {
    current_row: i32,
    current_col: i32,
    merged: DisjointSet,
    state: IState,
}

impl Eller {
    pub fn new(board: &Board) -> Self {
        Self {
            current_row: 0,
            current_col: 0,
            merged: DisjointSet::with_len(board.cells.len()),
            state: IState::Start,
        }
    }

    fn first_row<R: Rng>(&mut self, board: &mut Board, rng: &mut R) {
        let current_cell = board.get_index(self.current_col, self.current_row) as usize;
        let next_cell = board.get_index(self.current_col + 1, self.current_row) as usize;
        if !self.merged.is_joined(current_cell, next_cell) && rng.random_bool(BOOL_TRUE_PROBABILITY)
        {
            self.merged.join(current_cell, next_cell);
            board.cells[current_cell].walls.right = false;
            board.cells[next_cell].walls.left = false;
        }
        self.current_col += 1;
    }

    fn join_cells(&mut self, board: &mut Board) {
        let current_cell = board.get_index(self.current_col, self.current_row) as usize;
        let next_cell = board.get_index(self.current_col + 1, self.current_row) as usize;
        if !self.merged.is_joined(current_cell, next_cell) {
            self.merged.join(current_cell, next_cell);
            board.cells[current_cell].walls.right = false;
            board.cells[next_cell].walls.left = false;
        }
        self.current_col += 1;
    }

    fn open_bottom<R: Rng>(&mut self, board: &mut Board, mut rng: &mut R) {
        let mut cells = Vec::new();
        let first_cell = board.get_index(self.current_col, self.current_row) as usize;
        cells.push(first_cell);
        self.current_col += 1;
        while self.current_col < board.board_size
            && self.merged.is_joined(
                first_cell,
                board.get_index(self.current_col, self.current_row) as usize,
            )
        {
            cells.push(board.get_index(self.current_col, self.current_row) as usize);
            self.current_col += 1;
        }
        let mut opened = false;
        for index in &cells {
            if rng.random_bool(BOOL_TRUE_PROBABILITY) {
                let next_index =
                    board.get_index(board.cells[*index].x, board.cells[*index].y + 1) as usize;
                self.merged.join(*index, next_index);
                board.cells[*index].walls.bottom = false;
                board.cells[next_index].walls.top = false;
                opened = true;
            }
        }

        if !opened {
            let index = if cells.len() == 1 {
                cells[0]
            } else if let Some(&index) = cells.choose(&mut rng) {
                index
            } else {
                panic!("no item selected");
            };

            let next_index =
                board.get_index(board.cells[index].x, board.cells[index].y + 1) as usize;
            self.merged.join(index, next_index);
            board.cells[index].walls.bottom = false;
            board.cells[next_index].walls.top = false;
        }
        self.current_col -= 1;
    }

    fn last_row<R: Rng>(&mut self, board: &mut Board, rng: &mut R) {
        // self.join_cells(board, rng);
        for i in 0..board.board_size as usize {
            let current_cell = board.get_index(i as i32, self.current_row) as usize;
            if !self.merged.is_joined(0, current_cell) {
                self.merged.join(0, current_cell);
                if i == board.board_size as usize - 1 {
                    if rng.random_bool(BOOL_TRUE_PROBABILITY) {
                        let prev_cell = board.get_index(i as i32 - 1, self.current_row) as usize;
                        board.cells[current_cell].walls.left = false;
                        board.cells[prev_cell].walls.right = false;
                    } else {
                        let top_cell = board.get_index(i as i32, self.current_row - 1) as usize;
                        board.cells[current_cell].walls.top = false;
                        board.cells[top_cell].walls.bottom = false;
                    }
                } else {
                    let current_cell = board.get_index(i as i32, self.current_row) as usize;
                    let next_cell = board.get_index(i as i32 + 1, self.current_row) as usize;
                    let top_cell = board.get_index(i as i32, self.current_row - 1) as usize;
                    board.cells[current_cell].walls.right = false;
                    board.cells[next_cell].walls.left = false;
                    if i > 0 {
                        board.cells[current_cell].walls.left = false;
                        board.cells[next_cell].walls.right = false;
                    }
                    board.cells[current_cell].walls.top = false;
                    board.cells[top_cell].walls.bottom = false;
                }
            }
        }
    }
}

impl Generator for Eller {
    fn step(&mut self, board: &mut Board) -> State {
        let mut rng = rand::rng();
        match self.state {
            IState::Start => {
                self.first_row(board, &mut rng);
                if self.current_col >= board.board_size - 1 {
                    self.current_col = 0;
                    self.state = IState::Merge;
                }
            }
            IState::Merge => {
                self.join_cells(board);
                if self.current_col >= board.board_size - 1 {
                    self.current_col = 0;
                    self.state = IState::Bottom;
                }
            }
            IState::Bottom => {
                self.open_bottom(board, &mut rng);
                if self.current_col < board.board_size - 1 {
                    self.current_col += 1;
                } else {
                    self.current_col = 0;
                    self.current_row += 1;
                    if self.current_row >= board.board_size - 1 {
                        self.state = IState::Last;
                    } else {
                        self.state = IState::Merge;
                    }
                }
            }
            IState::Last => {
                self.last_row(board, &mut rng);
                self.state = IState::Finished;
            }
            IState::Finished => {
                return State::GenerationDone;
            }
        }

        State::Generate
    }
}
