use rand::prelude::*;

use crate::maze::{Board, CURSOR_COLOR, Generator, State};

#[derive(Default)]
pub struct Backtracking {
    current: i32,
}

impl Backtracking {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Generator for Backtracking {
    fn step(&mut self, board: &mut Board) -> State {
        if board.finish {
            return State::GenerationDone;
        }

        let n = board.neighbours(self.current);
        let free: Option<(usize, &Option<usize>)> = n
            .iter()
            .enumerate()
            .filter(|&(_, i)| i.is_some() && !board.cells[i.unwrap()].visited)
            .choose(&mut rand::rng());

        if let Some((index, Some(free))) = free {
            // remove the walls
            match index {
                0 => {
                    board.cells[self.current as usize].walls.top = false;
                    board.cells[*free].walls.bottom = false;
                }
                1 => {
                    board.cells[self.current as usize].walls.bottom = false;
                    board.cells[*free].walls.top = false;
                }
                2 => {
                    board.cells[self.current as usize].walls.left = false;
                    board.cells[*free].walls.right = false;
                }
                3 => {
                    board.cells[self.current as usize].walls.right = false;
                    board.cells[*free].walls.left = false;
                }
                _ => panic!("wrong index"),
            }

            // set next cell as current
            board.cells[*free].visited = true;
            self.current = *free as i32;
            board.path.push(*free)
        } else if let Some(last) = board.path.pop() {
            self.current = last as i32;
        } else {
            board.finish = true;
        }

        // draw the result
        use crate::raylib;
        unsafe {
            raylib::DrawCircle(
                board.x + board.cells[self.current as usize].x * board.cell_size,
                board.y + board.cells[self.current as usize].y * board.cell_size,
                board.cell_size as f32 / 2.0,
                CURSOR_COLOR,
            );
        }

        State::Generate
    }
}
