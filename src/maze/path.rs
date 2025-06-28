use crate::{
    maze::{Board, Cell, PATH_COLOR, djikstra::Solver},
    raylib,
};

#[derive(Debug)]
pub enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Horizontal,
    Vertical,
    StartLeft,
    StartDown,
    StartUp,
    StartRight,
    EndLeft,
    EndDown,
    EndUp,
    EndRight,
}

fn path_dot(x: i32, y: i32, cell: &Cell, cell_size: i32) {
    let half_cell = cell_size / 2;
    unsafe {
        raylib::DrawCircle(
            x + cell.x * cell_size + half_cell,
            y + cell.y * cell_size + half_cell,
            cell_size as f32 / 10.0,
            PATH_COLOR,
        );
    }
}

fn path_down(x: i32, y: i32, cell: &Cell, cell_size: i32) {
    let half_cell = cell_size / 2;
    unsafe {
        raylib::DrawLine(
            x + cell.x * cell_size + half_cell,
            y + cell.y * cell_size + half_cell,
            x + cell.x * cell_size + half_cell,
            y + cell.y * cell_size + cell_size,
            PATH_COLOR,
        );
    }
}
fn path_up(x: i32, y: i32, cell: &Cell, cell_size: i32) {
    let half_cell = cell_size / 2;
    unsafe {
        raylib::DrawLine(
            x + cell.x * cell_size + half_cell,
            y + cell.y * cell_size + half_cell,
            x + cell.x * cell_size + half_cell,
            y + cell.y * cell_size,
            PATH_COLOR,
        );
    }
}

fn path_left(x: i32, y: i32, cell: &Cell, cell_size: i32) {
    let half_cell = cell_size / 2;
    unsafe {
        raylib::DrawLine(
            x + cell.x * cell_size + half_cell,
            y + cell.y * cell_size + half_cell,
            x + cell.x * cell_size,
            y + cell.y * cell_size + half_cell,
            PATH_COLOR,
        );
    }
}

fn path_right(x: i32, y: i32, cell: &Cell, cell_size: i32) {
    let half_cell = cell_size / 2;
    unsafe {
        raylib::DrawLine(
            x + cell.x * cell_size + half_cell,
            y + cell.y * cell_size + half_cell,
            x + cell.x * cell_size + cell_size,
            y + cell.y * cell_size + half_cell,
            PATH_COLOR,
        );
    }
}

fn direction(current: &Cell, prev: Option<&Cell>, next: Option<&Cell>) -> Direction {
    // +---+---+---+    current.y < previous.y &&
    // +   +   +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // +   + c + n +    current.x < next.x
    // +---+---+---+
    // +   + p +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   +   +   +    current.y < next.y &&
    // +---+---+---+    current.x < previous.x &&
    // +   + c + p +    currnet.x == next.x
    // +---+---+---+
    // +   + n +   +
    // +---+---+---+
    //
    // +---+---+---+    current.y < previous.y &&
    // +   +   +   +    current.y == next.y &&
    // +---+---+---+    currnet.x == previous.x &&
    // + n + c +   +    current.x > next.x
    // +---+---+---+
    // +   + p +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   +   +   +    current.y < next.y &&
    // +---+---+---+    current.x > previous.x &&
    // + p + c +   +    current.x == next.x
    // +---+---+---+
    // +   + n +   +
    // +---+---+---+

    //
    // +---+---+---+    current.y > previous.y &&
    // +   + p +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // +   + c + n +    currnet.x < next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   + n +   +    current.y > next.y &&
    // +---+---+---+    current.x < prvious.x &&
    // +   + c + p +    current.x == next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    //
    // +---+---+---+    current.y > previous.y &&
    // +   + p +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // + n + c +   +    current.x > next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   + n +   +    current.y > next.y &&
    // +---+---+---+    current.x > previous.x &&
    // + p + c +   +    current.x == next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    //
    if let (Some(prev), Some(next)) = (prev, next) {
        if current.x == next.x && current.x == prev.x {
            return Direction::Vertical;
        } else if current.y == next.y && current.y == prev.y {
            return Direction::Horizontal;
        } else if (current.y < prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x < next.x)
            || (current.y == prev.y
                && current.y < next.y
                && current.x < prev.x
                && current.x == next.x)
        {
            return Direction::DownRight;
        } else if (current.y < prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x > next.x)
            || (current.y == prev.y
                && current.y < next.y
                && current.x > prev.x
                && current.x == next.x)
        {
            return Direction::DownLeft;
        } else if (current.y > prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x < next.x)
            || (current.y == prev.y
                && current.y > next.y
                && current.x < prev.x
                && current.x == next.x)
        {
            return Direction::UpRight;
        } else if (current.y > prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x > next.x)
            || (current.y == prev.y
                && current.y > next.y
                && current.x > prev.x
                && current.x == next.x)
        {
            return Direction::UpLeft;
        }
    } else if let Some(next) = next {
        if next.x > current.x {
            return Direction::EndRight;
        } else if next.x < current.x {
            return Direction::EndLeft;
        } else if next.y > current.y {
            return Direction::EndDown;
        } else if next.y < current.y {
            return Direction::EndUp;
        } else {
            panic!("direction not found")
        }
    } else if let Some(prev) = prev {
        if prev.x > current.x {
            return Direction::StartRight;
        } else if prev.x < current.x {
            return Direction::StartLeft;
        } else if prev.y > current.y {
            return Direction::StartDown;
        } else if prev.y < current.y {
            return Direction::StartUp;
        } else {
            panic!("direction not found")
        }
    }
    panic!("direction not found")
}

pub fn draw_path(board: &Board, solver: &Solver) {
    // draw the path
    let x = board.x;
    let y = board.y;
    unsafe {
        for (i, item) in solver.path.iter().enumerate() {
            let prev = if i > 0 { solver.path.get(i - 1) } else { None };
            let next = solver.path.get(i + 1); // get handles out-of-bounds by returning None
            let direction = direction(
                &board.cells[*item],
                if let Some(prev) = prev {
                    Some(&board.cells[*prev])
                } else {
                    None
                },
                if let Some(next) = next {
                    Some(&board.cells[*next])
                } else {
                    None
                },
            );
            match direction {
                Direction::Horizontal => {
                    raylib::DrawLine(
                        x + board.cells[*item].x * board.cell_size,
                        y + board.cells[*item].y * board.cell_size + board.cell_size / 2,
                        x + board.cells[*item].x * board.cell_size + board.cell_size,
                        y + board.cells[*item].y * board.cell_size + board.cell_size / 2,
                        PATH_COLOR,
                    );
                }
                Direction::Vertical => {
                    raylib::DrawLine(
                        x + board.cells[*item].x * board.cell_size + board.cell_size / 2,
                        y + board.cells[*item].y * board.cell_size,
                        x + board.cells[*item].x * board.cell_size + board.cell_size / 2,
                        y + board.cells[*item].y * board.cell_size + board.cell_size,
                        PATH_COLOR,
                    );
                }
                Direction::StartLeft | Direction::EndLeft => {
                    path_dot(x, y, &board.cells[*item], board.cell_size);
                    path_left(x, y, &board.cells[*item], board.cell_size);
                }
                Direction::StartRight | Direction::EndRight => {
                    path_dot(x, y, &board.cells[*item], board.cell_size);
                    path_right(x, y, &board.cells[*item], board.cell_size);
                }
                Direction::StartUp | Direction::EndUp => {
                    path_dot(x, y, &board.cells[*item], board.cell_size);
                    path_up(x, y, &board.cells[*item], board.cell_size);
                }
                Direction::StartDown | Direction::EndDown => {
                    path_dot(x, y, &board.cells[*item], board.cell_size);
                    path_down(x, y, &board.cells[*item], board.cell_size);
                }
                Direction::UpLeft => {
                    path_up(x, y, &board.cells[*item], board.cell_size);
                    path_left(x, y, &board.cells[*item], board.cell_size);
                }
                Direction::UpRight => {
                    path_up(x, y, &board.cells[*item], board.cell_size);
                    path_right(x, y, &board.cells[*item], board.cell_size);
                }
                Direction::DownLeft => {
                    path_down(x, y, &board.cells[*item], board.cell_size);
                    path_left(x, y, &board.cells[*item], board.cell_size);
                }
                Direction::DownRight => {
                    path_down(x, y, &board.cells[*item], board.cell_size);
                    path_right(x, y, &board.cells[*item], board.cell_size);
                }
            }
        }
    }
}
