use rand::prelude::*;

pub mod djikstra;

pub const TOP: u8 = 0b00001000;
pub const LEFT: u8 = 0b00000100;
pub const BOTTOM: u8 = 0b00000010;
pub const RIGHT: u8 = 0b00000001;

#[derive(Clone, Debug)]
pub struct Walls {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl Default for Walls {
    fn default() -> Self {
        Self {
            left: true,
            right: true,
            top: true,
            bottom: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub visited: bool,
    pub walls: Walls,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            visited: false,
            walls: Walls::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub path: Vec<usize>,
    pub current: i32,
    pub board_size: i32,
    pub finish: bool,
}

impl Board {
    pub fn new(mut cells: Vec<Cell>, board_size: i32) -> Self {
        cells[0].visited = true;
        cells[0].walls.left = false;
        cells.last_mut().unwrap().walls.right = false;
        Self {
            cells,
            path: vec![0],
            current: 0,
            board_size,
            finish: false,
        }
    }
    pub fn get_current(&self) -> &Cell {
        &self.cells[self.current as usize]
    }

    /**
     * return the neighbours [top, bottom, right, left]
     */
    pub fn neighbours(&self) -> Vec<Option<usize>> {
        let mut res = Vec::<Option<usize>>::new();
        if self.cells[self.current as usize].y > 1 {
            res.push(Some(self.current as usize - 1));
        } else {
            res.push(None);
        }
        if self.cells[self.current as usize].y < self.board_size - 1 {
            res.push(Some(self.current as usize + 1));
        } else {
            res.push(None);
        }
        if self.cells[self.current as usize].x > 1 {
            res.push(Some(self.current as usize - self.board_size as usize + 1));
        } else {
            res.push(None);
        }
        if self.cells[self.current as usize].x < self.board_size - 1 {
            res.push(Some(self.current as usize + self.board_size as usize - 1));
        } else {
            res.push(None);
        }
        res
    }

    pub fn step(&mut self) {
        let n = self.neighbours();
        let free: Option<(usize, &Option<usize>)> = n
            .iter()
            .enumerate()
            .filter(|&(_, i)| i.is_some() && !self.cells[i.unwrap()].visited)
            .choose(&mut rand::rng());

        // if let Some(free) = free {
        if let Some((index, Some(free))) = free {
            // remove the walls
            match index {
                0 => {
                    self.cells[self.current as usize].walls.top = false;
                    self.cells[*free].walls.bottom = false;
                }
                1 => {
                    self.cells[self.current as usize].walls.bottom = false;
                    self.cells[*free].walls.top = false;
                }
                2 => {
                    self.cells[self.current as usize].walls.left = false;
                    self.cells[*free].walls.right = false;
                }
                3 => {
                    self.cells[self.current as usize].walls.right = false;
                    self.cells[*free].walls.left = false;
                }
                _ => panic!("wrong index"),
            }

            // set next cell as current
            self.cells[*free].visited = true;
            self.current = *free as i32;
            self.path.push(*free)
        } else if let Some(last) = self.path.pop() {
            self.current = last as i32;
        } else {
            self.finish = true;
        }
    }
}
