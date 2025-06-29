use std::{collections::HashMap, ffi::CString};

use disjoint::DisjointSet;
use rand::prelude::*;

use crate::{
    maze::{Board, Generator, State},
    raylib,
};
pub const BOOL_TRUE_PROBABILITY: f64 = 0.5;

// #[derive(Default, Debug)]
// struct Cell {
//     x: i32,
//     y: i32,
// }
//
// #[derive(Default)]
// struct Sets {
//     row_num: usize,
//     cells: HashMap<i32, Cell>,
// }

#[derive(Default)]
pub struct Eller {
    current_row: i32,
    merged: DisjointSet,
}

impl Eller {
    pub fn new(board: &Board) -> Self {
        Self {
            current_row: 0,
            merged: DisjointSet::with_len(board.cells.len()),
        }
    }

    fn join_cells<R: Rng>(&mut self, board: &mut Board, rng: &mut R) {
        for i in 0..board.board_size as usize - 1 {
            let current_cell = board.get_index(i as i32, self.current_row) as usize;
            let next_cell = board.get_index(i as i32 + 1, self.current_row) as usize;
            if !self.merged.is_joined(current_cell, next_cell)
                && rng.random_bool(BOOL_TRUE_PROBABILITY)
            {
                self.merged.join(current_cell, next_cell);
                board.cells[current_cell].walls.right = false;
                board.cells[next_cell].walls.left = false;
            }
        }
    }

    fn open_bottom<R: Rng>(&mut self, board: &mut Board, mut rng: &mut R) {
        let mut i = 0;
        while i < board.board_size as usize {
            let mut cells = Vec::new();
            let first_cell = board.get_index(i as i32, self.current_row) as usize;
            cells.push(first_cell);
            i += 1;
            while i < board.board_size as usize
                && self.merged.is_joined(
                    first_cell,
                    board.get_index(i as i32, self.current_row) as usize,
                )
            {
                cells.push(board.get_index(i as i32, self.current_row) as usize);
                i += 1;
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
        }
    }

    fn last_row<R: Rng>(&mut self, board: &mut Board, rng: &mut R) {
        self.join_cells(board, rng);
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
        if self.current_row >= board.board_size {
            // return State::GenerationDone;
        }

        if self.current_row < board.board_size - 1 {
            self.join_cells(board, &mut rng);
            if self.current_row < board.board_size - 1 {
                self.open_bottom(board, &mut rng);
            }
        } else if self.current_row == board.board_size - 1 {
            self.last_row(board, &mut rng);
            println!("finished: sets: {}", self.merged.sets().len());
        }

        let set_count = self.merged.sets().len();
        let mut current_set = 1;
        for (i, set) in self.merged.sets().iter().enumerate() {
            unsafe {
                if set.len() > 1 {
                    for index in set {
                        raylib::DrawRectangle(
                            board.x + board.cells[*index].x * board.cell_size + 2,
                            board.y + board.cells[*index].y * board.cell_size + 2,
                            board.cell_size - 4,
                            board.cell_size - 4,
                            raylib::ColorFromHSV(
                                100.0,
                                0.75,
                                1.0 / set_count as f32 * current_set as f32,
                            ),
                        );
                    }
                }
                for index in set {
                    raylib::DrawText(
                        CString::new(format!("{}", i)).expect("cstr").as_ptr(),
                        board.x + board.cell_size * board.cells[*index].x,
                        board.y + board.cell_size * board.cells[*index].y,
                        12,
                        raylib::WHITE,
                    );
                }
            }
            current_set += 1;
        }

        self.current_row += 1;
        State::Generate
    }
}
