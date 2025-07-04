use rand::prelude::*;

use crate::maze::{Board, Generator, State};

#[derive(Debug)]
struct Area {
    start: (i32, i32),
    end: (i32, i32),
}

pub struct RecursiveDivision {
    areas: Vec<Area>,
    step: usize,
    rng: ThreadRng,
    probability: f64,
}

impl RecursiveDivision {
    pub fn new(board: &mut Board) -> Self {
        //remove all walls
        for cell in &mut board.cells {
            if cell.x > 0 {
                cell.walls.left = false;
            }
            if cell.y > 0 {
                cell.walls.top = false;
            }
            if cell.x < board.board_size - 1 {
                cell.walls.right = false;
            }
            if cell.y < board.board_size - 1 {
                cell.walls.bottom = false;
            }
            cell.visited = true;
        }
        Self {
            areas: vec![Area {
                start: (0, 0),
                end: (board.board_size, board.board_size),
            }],
            step: 0,
            rng: rand::rng(),
            probability: 0.5,
        }
    }
}

impl Generator for RecursiveDivision {
    fn step(&mut self, board: &mut Board) -> State {
        println!("------------------");
        let mut new_areas: Vec<Area> = Vec::new();
        for area in &self.areas {
            println!("area: {:?}", area);
            if self.rng.random_bool(self.probability) {
                // horzontal
                let y = self.rng.random_range(area.start.1..area.end.1 - 1) as usize;
                let x = self.rng.random_range(area.start.0..area.end.0) as usize;
                println!(
                    "horizontal: start y: {}, end y: {}, x: {}, y: {}",
                    area.start.1, area.end.1, x, y
                );
                for index in area.start.0..area.end.0 {
                    if x != index as usize {
                        let c0 = board.get_index(index, y as i32);
                        board.cells[c0 as usize].walls.bottom = true;
                        if y < board.board_size as usize - 1 {
                            let c1 = board.get_index(index, y as i32 + 1);
                            board.cells[c1 as usize].walls.top = true;
                        }
                    }
                }

                //size is bigger then 1 cell
                if y - area.start.1 as usize > 0 {
                    new_areas.push(Area {
                        start: (area.start.0, area.start.1),
                        end: (area.end.0, y as i32 + 1),
                    });
                }
                if area.end.1 as usize - y - 1 > 1 {
                    new_areas.push(Area {
                        start: (area.start.0, y as i32 + 1),
                        end: (area.end.0, area.end.1),
                    });
                }
            } else {
                // vertical
                let y = self.rng.random_range(area.start.1..area.end.1) as usize;
                let x = self.rng.random_range(area.start.0..area.end.0) as usize;
                println!(
                    "vertical: start y: {}, end y: {}, x: {}, y: {}",
                    area.start.1, area.end.1, x, y
                );

                for index in area.start.1..area.end.1 {
                    if y != index as usize {
                        let c0 = board.get_index(x as i32, index);
                        board.cells[c0 as usize].walls.right = true;
                        if x < board.board_size as usize - 1 {
                            let c1 = board.get_index(x as i32 + 1, index);
                            board.cells[c1 as usize].walls.left = true;
                        }
                    }
                }
                //size is bigger then 1 cell
                if x - area.start.0 as usize > 0 {
                    new_areas.push(Area {
                        start: (area.start.0, area.start.1),
                        end: (x as i32, area.end.1),
                    });
                }
                if area.end.0 as usize - x > 1 {
                    new_areas.push(Area {
                        start: (x as i32 + 1, area.start.1),
                        end: (area.end.0, area.end.1),
                    });
                }
            }
        }

        self.areas = new_areas;

        self.step += 1;
        if self.step > 10 {
            State::GenerationDone
        } else {
            State::Generate
        }
    }

    fn draw(&self, _board: &Board) {}
}
