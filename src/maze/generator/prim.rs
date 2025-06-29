use rand::prelude::*;

use crate::{
    maze::{Board, CURSOR_COLOR, Generator, State},
    raylib,
};

#[derive(Debug)]
struct FreeCell {
    index: usize,
    neighbour: usize,
}

pub struct Prim {
    visited: Vec<usize>,
    cells: Vec<FreeCell>,
}

impl Prim {
    pub fn new(board: &Board) -> Self {
        let current = rand::rng().random_range(0..board.board_size ^ 2) as usize;
        let cells = board
            .neighbours(current as i32)
            .into_iter()
            .flatten()
            .map(|index| FreeCell {
                index,
                neighbour: current,
            })
            .collect();

        Self {
            visited: vec![current],
            cells,
        }
    }

    fn contains(&self, index: &usize) -> bool {
        self.cells
            .iter()
            .filter(|item| item.index == *index)
            .count()
            > 0
            || self.visited.contains(index)
    }
}

impl Generator for Prim {
    fn step(&mut self, board: &mut Board) -> State {
        let index = rand::rng().random_range(0..self.cells.len()) as usize;
        let item = self.cells.remove(index);
        // remove wall
        match board.cells[item.index].direction(&board.cells[item.neighbour]) {
            crate::maze::Direction::North => {
                board.cells[item.index].walls.top = false;
                board.cells[item.neighbour].walls.bottom = false;
            }
            crate::maze::Direction::South => {
                board.cells[item.index].walls.bottom = false;
                board.cells[item.neighbour].walls.top = false;
            }
            crate::maze::Direction::East => {
                board.cells[item.index].walls.right = false;
                board.cells[item.neighbour].walls.left = false;
            }
            crate::maze::Direction::West => {
                board.cells[item.index].walls.left = false;
                board.cells[item.neighbour].walls.right = false;
            }
        }

        // calc next cells
        let neighbours: Vec<usize> = board
            .neighbours(item.index as i32)
            .into_iter()
            .flatten()
            .collect();
        for n in &neighbours {
            if !self.contains(n) {
                self.cells.push(FreeCell {
                    index: *n,
                    neighbour: item.index,
                });
            }
        }

        self.visited.push(item.index);

        // draw the next cells
        for i in &self.cells {
            unsafe {
                raylib::DrawCircle(
                    board.x + board.cells[i.index].x * board.cell_size + board.cell_size / 2,
                    board.y + board.cells[i.index].y * board.cell_size + board.cell_size / 2,
                    board.cell_size as f32 / 4.0,
                    CURSOR_COLOR,
                );
            }
        }

        if self.cells.is_empty() {
            State::GenerationDone
        } else {
            State::Generate
        }
    }
}
