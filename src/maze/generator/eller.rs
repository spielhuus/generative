use std::collections::HashMap;

use disjoint::DisjointSet;
use rand::prelude::*;

use crate::maze::{Board, CURSOR_COLOR, Generator, State};
pub const BOOL_TRUE_PROBABILITY: f64 = 0.5;

enum IState {
    Merge,
    Bottom,
    Last,
}

pub struct Eller {
    x: i32,
    y: i32,
    merged: DisjointSet,
    state: IState,
    row: HashMap<usize, Vec<usize>>,
}

impl Eller {
    pub fn new(board: &Board) -> Self {
        Self {
            x: 0,
            y: 0,
            merged: DisjointSet::with_len(board.cells.len()),
            state: IState::Merge,
            row: HashMap::new(),
        }
    }
}

impl Generator for Eller {
    fn step(&mut self, board: &mut Board) -> State {
        let mut rng = rand::rng();

        match self.state {
            IState::Merge => {
                let cell = board.get_index(self.x, self.y) as usize;
                let neighbor = board.get_index(self.x + 1, self.y) as usize;

                if !self.merged.is_joined(cell, neighbor)
                    && (rng.random_bool(BOOL_TRUE_PROBABILITY) || self.y >= board.board_size - 1)
                {
                    self.merged.join(cell, neighbor);
                    board.remove_wall(cell, neighbor);
                }

                self.row
                    .entry(self.merged.root_of(cell))
                    .or_default()
                    .push(cell);

                self.x += 1;
                if self.x >= board.board_size - 1 {
                    self.row
                        .entry(
                            self.merged
                                .root_of(board.get_index(self.x, self.y) as usize),
                        )
                        .or_default()
                        .push(board.get_index(self.x, self.y) as usize);
                    self.x = 0;
                    if self.y == board.board_size - 1 {
                        self.state = IState::Last;
                    } else {
                        self.state = IState::Bottom;
                    }
                }
                State::Generate
            }
            IState::Bottom => {
                let cell = board.get_index(self.x, self.y) as usize;
                let neighbor = board.get_index(self.x, self.y + 1) as usize;
                if !self.merged.is_joined(cell, neighbor) && rng.random_bool(BOOL_TRUE_PROBABILITY)
                {
                    self.merged.join(cell, neighbor);
                    board.remove_wall(cell, neighbor);
                    self.row.remove(&self.merged.root_of(cell));
                }

                self.x += 1;
                if self.x >= board.board_size {
                    for cells in self.row.values() {
                        if let Some(&index) = cells.choose(&mut rng) {
                            let neighbor =
                                board.get_index(board.cells[index].x, board.cells[index].y + 1);
                            board.remove_wall(index, neighbor as usize);
                        } else {
                            panic!("no top neigbhor");
                        }
                    }
                    self.row.clear();
                    self.x = 0;
                    self.y += 1;
                    self.state = IState::Merge;
                }
                State::Generate
            }
            IState::Last => {
                let cell = board.get_index(self.x, self.y) as usize;
                let neighbor = board.get_index(self.x, self.y - 1) as usize;
                if !self.merged.is_joined(cell, neighbor) {
                    self.merged.join(cell, neighbor);
                    board.remove_wall(cell, neighbor);
                    self.row.remove(&self.merged.root_of(cell));
                }

                self.x += 1;
                if self.x >= board.board_size - 1 {
                    for cells in self.row.values() {
                        if let Some(&index) = cells.choose(&mut rng) {
                            let neighbor =
                                board.get_index(board.cells[index].x, board.cells[index].y - 1);
                            board.remove_wall(index, neighbor as usize);
                        } else {
                            panic!("no top neigbhor");
                        }
                    }
                    self.row.clear();
                }
                State::GenerationDone
            }
        }
    }

    fn draw(&self, _board: &Board) {}
}
