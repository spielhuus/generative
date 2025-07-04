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
        let n = board.neighbors(self.current);
        let free: Option<&Option<usize>> = n
            .iter()
            .filter(|i| i.is_some() && !board.cells[i.unwrap()].visited)
            .choose(&mut rand::rng());

        if let Some(&Some(free)) = free {
            // remove the walls
            board.remove_wall(self.current as usize, free);
            // set next cell as current
            board.cells[free].visited = true;
            self.current = free as i32;
            board.path.push(free)
        } else if let Some(last) = board.path.pop() {
            self.current = last as i32;
        } else {
            return State::GenerationDone;
        }

        State::Generate
    }

    fn draw(&self, board: &Board) {
        // draw the result
        use crate::raylib;
        unsafe {
            raylib::DrawCircle(
                board.x
                    + board.cells[self.current as usize].x * board.cell_size
                    + board.cell_size / 2,
                board.y
                    + board.cells[self.current as usize].y * board.cell_size
                    + board.cell_size / 2,
                board.cell_size as f32 / 10.0,
                CURSOR_COLOR,
            );
        }
    }
}
