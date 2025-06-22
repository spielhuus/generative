use generative::{
    maze::{Board, Cell, djikstra},
    raygui, raylib,
};

use std::ffi::{CString, c_char};

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 800;
const CELL_SIZE: i32 = 40;
const TITLE: &str = "";

const WALL_COLOR: raylib::Color = raylib::Color {
    r: 100,
    g: 100,
    b: 100,
    a: 255,
};
const PATH_COLOR: raylib::Color = raylib::Color {
    r: 100,
    g: 255,
    b: 100,
    a: 255,
};
const CURSOR_COLOR: raylib::Color = raylib::Color {
    r: 125,
    g: 0,
    b: 17,
    a: 255,
};

#[derive(Debug)]
enum Direction {
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

fn path_dot(cell: &Cell, cell_size: i32) {
    unsafe {
        raylib::DrawCircle(
            cell.x * cell_size,
            cell.y * cell_size,
            cell_size as f32 / 10.0,
            PATH_COLOR,
        );
    }
}

fn path_down(cell: &Cell, cell_size: i32) {
    unsafe {
        raylib::DrawLine(
            cell.x * cell_size,
            cell.y * cell_size,
            cell.x * cell_size,
            cell.y * cell_size + cell_size / 2,
            PATH_COLOR,
        );
    }
}
fn path_up(cell: &Cell, cell_size: i32) {
    unsafe {
        raylib::DrawLine(
            cell.x * cell_size,
            cell.y * cell_size,
            cell.x * cell_size,
            cell.y * cell_size - cell_size / 2,
            PATH_COLOR,
        );
    }
}

fn path_left(cell: &Cell, cell_size: i32) {
    unsafe {
        raylib::DrawLine(
            cell.x * cell_size,
            cell.y * cell_size,
            cell.x * cell_size - cell_size / 2,
            cell.y * cell_size,
            PATH_COLOR,
        );
    }
}

fn path_right(cell: &Cell, cell_size: i32) {
    unsafe {
        raylib::DrawLine(
            cell.x * cell_size,
            cell.y * cell_size,
            cell.x * cell_size + cell_size / 2,
            cell.y * cell_size,
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

fn main() {
    unsafe {
        let mut cell_size = CELL_SIZE;

        let mut text_buffer: Vec<u8> = vec![];
        text_buffer.extend_from_slice(format!("{}", cell_size).as_bytes());
        text_buffer.push(0);
        println!("{:?}", text_buffer);

        raylib::InitWindow(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            CString::new(TITLE).expect("cstr").as_ptr(),
        );

        raylib::SetTargetFPS(240);

        // set raygui style
        raygui::GuiSetStyle(
            raygui::GuiControl::Default,
            raygui::GuiControlProperty::TextSize,
            18,
        );

        let mut grid_size = if SCREEN_WIDTH / cell_size > SCREEN_HEIGHT / cell_size {
            SCREEN_HEIGHT / cell_size
        } else {
            SCREEN_WIDTH / cell_size
        };

        let mut cells = vec![];
        for i in 1..grid_size {
            for j in 1..grid_size {
                cells.push(Cell::new(i, j));
            }
        }
        let mut board = Board::new(cells, grid_size);
        let mut solver = djikstra::Solver::new(
            (1, 1),
            (grid_size as usize - 1, grid_size as usize - 1),
            grid_size as usize,
        );

        let mut generate = false;
        let mut solve = false;

        // main loop
        while !raylib::WindowShouldClose() {
            raylib::BeginDrawing();
            raylib::ClearBackground(raylib::BLACK);

            // draw the ui
            raylib::DrawText(
                CString::new("Grid Size: ").expect("cstr").as_ptr(),
                SCREEN_WIDTH - 300,
                70,
                24,
                raylib::WHITE,
            );

            if raygui::GuiTextBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 150.0,
                    y: 65.0,
                    width: 80.0,
                    height: 30.0,
                },
                text_buffer.as_mut_ptr() as *mut c_char,
                4,
                true,
            ) != 0
            {
                let res: Vec<u8> = text_buffer
                    .iter()
                    .filter(|c| (**c as char).is_ascii_digit())
                    .cloned()
                    .collect();

                let new_cell_size = std::str::from_utf8(res.as_slice())
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();

                if new_cell_size != cell_size {
                    cell_size = new_cell_size;
                    grid_size = if SCREEN_WIDTH / cell_size > SCREEN_HEIGHT / cell_size {
                        SCREEN_HEIGHT / cell_size
                    } else {
                        SCREEN_WIDTH / cell_size
                    };

                    let mut cells = vec![];
                    for i in 1..grid_size {
                        for j in 1..grid_size {
                            cells.push(Cell::new(i, j));
                        }
                    }
                    board = Board::new(cells, grid_size);
                    solver = djikstra::Solver::new(
                        (1, 1),
                        (grid_size as usize - 1, grid_size as usize - 1),
                        grid_size as usize,
                    );
                    generate = false;
                    solve = false;
                }
            }

            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 300.0,
                    y: 300.0,
                    width: 80.0,
                    height: 30.0,
                },
                CString::new("generate").expect("cstr").as_ptr(),
            ) != 0
            {
                generate = true;
            }

            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 200.0,
                    y: 300.0,
                    width: 80.0,
                    height: 30.0,
                },
                CString::new("solve").expect("cstr").as_ptr(),
            ) != 0
            {
                solve = true;
            }

            // draw the board
            for cell in &board.cells {
                let x = cell.x * cell_size - cell_size / 2;
                let y = cell.y * cell_size - cell_size / 2;
                if cell.walls.top {
                    raylib::DrawLine(x, y, x + cell_size, y, WALL_COLOR);
                }
                if cell.walls.right {
                    raylib::DrawLine(x + cell_size, y, x + cell_size, y + cell_size, WALL_COLOR);
                }
                if cell.walls.bottom {
                    raylib::DrawLine(x + cell_size, y + cell_size, x, y + cell_size, WALL_COLOR);
                }
                if cell.walls.left {
                    raylib::DrawLine(x, y + cell_size, x, y, WALL_COLOR);
                }
            }

            if generate && !board.finish {
                raylib::DrawCircle(
                    board.get_current().x * cell_size, // - cell_size / 4,
                    board.get_current().y * cell_size, // - cell_size / 4,
                    cell_size as f32 / 2.0,
                    CURSOR_COLOR,
                );
                board.step();
            } else if solve && !solver.reached_end {
                solver.step(&board);
                for weight in solver.weights.iter().flatten() {
                    raylib::DrawText(
                        CString::new(std::format!("{}", weight.weight))
                            .expect("cstr")
                            .as_ptr(),
                        weight.x as i32 * cell_size - cell_size / 3,
                        weight.y as i32 * cell_size - cell_size / 3,
                        12,
                        raylib::WHITE,
                    );
                }
            } else if solve && !solver.solved {
                solver.path(&board);
                for (index, weight) in solver.weights.iter().enumerate() {
                    if let Some(weight) = weight {
                        if solver.path.contains(&index) {
                            raylib::DrawText(
                                CString::new(std::format!("{}", weight.weight))
                                    .expect("cstr")
                                    .as_ptr(),
                                weight.x as i32 * cell_size - cell_size / 3,
                                weight.y as i32 * cell_size - cell_size / 3,
                                12,
                                raylib::WHITE,
                            );
                        } else {
                            raylib::DrawText(
                                CString::new(std::format!("{}", weight.weight))
                                    .expect("cstr")
                                    .as_ptr(),
                                weight.x as i32 * cell_size - cell_size / 3,
                                weight.y as i32 * cell_size - cell_size / 3,
                                12,
                                raylib::GREY,
                            );
                        }
                    }
                }
            } else {
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
                                board.cells[*item].x * cell_size - cell_size / 2,
                                board.cells[*item].y * cell_size,
                                board.cells[*item].x * cell_size + cell_size / 2,
                                board.cells[*item].y * cell_size,
                                PATH_COLOR,
                            );
                        }
                        Direction::Vertical => {
                            raylib::DrawLine(
                                board.cells[*item].x * cell_size,
                                board.cells[*item].y * cell_size - cell_size / 2,
                                board.cells[*item].x * cell_size,
                                board.cells[*item].y * cell_size + cell_size / 2,
                                PATH_COLOR,
                            );
                        }
                        Direction::StartLeft | Direction::EndLeft => {
                            path_dot(&board.cells[*item], cell_size);
                            path_left(&board.cells[*item], cell_size);
                        }
                        Direction::StartRight | Direction::EndRight => {
                            path_dot(&board.cells[*item], cell_size);
                            path_right(&board.cells[*item], cell_size);
                        }
                        Direction::StartUp | Direction::EndUp => {
                            path_dot(&board.cells[*item], cell_size);
                            path_up(&board.cells[*item], cell_size);
                        }
                        Direction::StartDown | Direction::EndDown => {
                            path_dot(&board.cells[*item], cell_size);
                            path_down(&board.cells[*item], cell_size);
                        }
                        Direction::UpLeft => {
                            path_up(&board.cells[*item], cell_size);
                            path_left(&board.cells[*item], cell_size);
                        }
                        Direction::UpRight => {
                            path_up(&board.cells[*item], cell_size);
                            path_right(&board.cells[*item], cell_size);
                        }
                        Direction::DownLeft => {
                            path_down(&board.cells[*item], cell_size);
                            path_left(&board.cells[*item], cell_size);
                        }
                        Direction::DownRight => {
                            path_down(&board.cells[*item], cell_size);
                            path_right(&board.cells[*item], cell_size);
                        }
                    }
                }
            }

            raylib::EndDrawing();
        }
        raylib::CloseWindow();
    }
}
