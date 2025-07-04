use rand::prelude::*;

use crate::{
    maze::{Board, CURSOR_COLOR, Generator, State},
    raylib,
};

pub struct Wilson {
    visited: Vec<usize>,
    current_cell: usize,
}

impl Wilson {
    pub fn new(board: &Board) -> Self {
        let current_cell = rand::rng().random_range(0..board.board_size ^ 2) as usize;
        Self {
            visited: vec![current_cell],
            current_cell,
        }
    }

    fn contains(&self, index: &usize) -> bool {
        self.visited.contains(index)
    }
}

impl Generator for Wilson {
    fn step(&mut self, board: &mut Board) -> State {
        todo!("not implemented");
        // get the neighbours of the current cell and pick a random neighbour
        let neighbours: Vec<usize> = board
            .neighbors(self.current_cell as i32)
            .into_iter()
            .flatten()
            .collect();
        let index = rand::rng().random_range(0..neighbours.len()) as usize;
        let next = neighbours[index];
        // remove wall
        if !self.contains(&next) {
            match board.cells[self.current_cell].direction(&board.cells[next]) {
                crate::maze::Direction::North => {
                    board.cells[self.current_cell].walls.top = false;
                    board.cells[next].walls.bottom = false;
                }
                crate::maze::Direction::South => {
                    board.cells[self.current_cell].walls.bottom = false;
                    board.cells[next].walls.top = false;
                }
                crate::maze::Direction::East => {
                    board.cells[self.current_cell].walls.right = false;
                    board.cells[next].walls.left = false;
                }
                crate::maze::Direction::West => {
                    board.cells[self.current_cell].walls.left = false;
                    board.cells[next].walls.right = false;
                }
            }
            self.visited.push(next);
        }
        self.current_cell = next;

        if self.visited.len() >= board.cells.len() {
            State::GenerationDone
        } else {
            State::Generate
        }
    }

    fn draw(&self, board: &Board) {
        unsafe {
            raylib::DrawCircle(
                board.x + board.cells[self.current_cell].x * board.cell_size + board.cell_size / 2,
                board.y + board.cells[self.current_cell].y * board.cell_size + board.cell_size / 2,
                board.cell_size as f32 / 4.0,
                CURSOR_COLOR,
            );
        }
    }
}
