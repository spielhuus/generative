pub mod djikstra;
pub mod generator;
pub mod path;

use crate::raylib;

pub const WALL_COLOR: raylib::Color = raylib::Color {
    r: 100,
    g: 100,
    b: 100,
    a: 255,
};
pub const PATH_COLOR: raylib::Color = raylib::Color {
    r: 100,
    g: 255,
    b: 100,
    a: 255,
};
pub const CURSOR_COLOR: raylib::Color = raylib::Color {
    r: 125,
    g: 0,
    b: 17,
    a: 255,
};

#[derive(Debug, Copy, Clone)]
pub enum State {
    Wait,
    Generate,
    GenerationDone,
    Solve,
    Done,
}

pub trait Generator {
    fn step(&mut self, board: &mut Board) -> State;
}

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
    pub board_size: i32,
    pub finish: bool,
    pub cell_size: i32,
    pub x: i32,
    pub y: i32,
}

impl Board {
    pub fn new(screen_width: i32, screen_height: i32, cell_size: i32) -> Self {
        let mut board = Self {
            cells: Vec::new(),
            path: vec![0],
            board_size: 0,
            finish: false,
            cell_size,
            x: 0,
            y: 0,
        };
        board.init(screen_width, screen_height);
        board
    }

    fn init(&mut self, screen_width: i32, screen_height: i32) {
        self.board_size = if screen_width / self.cell_size > screen_height / self.cell_size {
            screen_height / self.cell_size - 1
        } else {
            screen_width / self.cell_size - 1
        };

        self.x = 5;
        self.y = (screen_height - self.board_size * self.cell_size) / 2;

        for i in 0..self.board_size {
            for j in 0..self.board_size {
                self.cells.push(Cell::new(i, j));
            }
        }
        self.cells[0].visited = true;
        self.cells[0].walls.left = false;
        self.cells.last_mut().unwrap().walls.right = false;
    }

    pub fn get_cell(&mut self, index: usize) -> &mut Cell {
        &mut self.cells[index]
    }

    pub fn get_index(&self, x: i32, y: i32) -> i32 {
        let index = x * self.board_size + y;
        assert!(self.cells[index as usize].x == x && self.cells[index as usize].y == y,);
        index
    }

    /**
     * return the neighbours [top, bottom, right, left]
     */
    pub fn neighbours(&self, cell_index: i32) -> Vec<Option<usize>> {
        let mut res = Vec::<Option<usize>>::new();
        if self.cells[cell_index as usize].y > 0 {
            res.push(Some(cell_index as usize - 1));
        } else {
            res.push(None);
        }
        if self.cells[cell_index as usize].y < self.board_size - 1 {
            res.push(Some(cell_index as usize + 1));
        } else {
            res.push(None);
        }
        if self.cells[cell_index as usize].x > 0 {
            res.push(Some(cell_index as usize - self.board_size as usize));
        } else {
            res.push(None);
        }
        if self.cells[cell_index as usize].x < self.board_size - 1 {
            res.push(Some(cell_index as usize + self.board_size as usize));
        } else {
            res.push(None);
        }
        res
    }

    pub fn draw(&self) {
        unsafe {
            for cell in &self.cells {
                let x = self.x + cell.x * self.cell_size;
                let y = self.y + cell.y * self.cell_size;
                if cell.walls.top {
                    raylib::DrawLine(x, y, x + self.cell_size, y, WALL_COLOR);
                }
                if cell.walls.right {
                    raylib::DrawLine(
                        x + self.cell_size,
                        y,
                        x + self.cell_size,
                        y + self.cell_size,
                        WALL_COLOR,
                    );
                }
                if cell.walls.bottom {
                    raylib::DrawLine(
                        x + self.cell_size,
                        y + self.cell_size,
                        x,
                        y + self.cell_size,
                        WALL_COLOR,
                    );
                }
                if cell.walls.left {
                    raylib::DrawLine(x, y + self.cell_size, x, y, WALL_COLOR);
                }
            }
        }
    }
}
