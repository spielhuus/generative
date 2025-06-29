use generative::{
    maze::{
        Board, Generator, State,
        djikstra::{self, Solver},
        generator::{backtracking::Backtracking, eller::Eller, kruskal::Kruskal},
        path,
    },
    raygui, raylib, str,
};

use std::ffi::{CString, c_char};

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 800;
const CELL_SIZE: i32 = 40;
const TITLE: &str = "";

static mut STATE: State = State::Wait;
static mut SELECTED_GENERATOR: i32 = 0;

fn init_maze(cell_size: i32) -> (Board, Box<dyn Generator>, Solver) {
    unsafe {
        let board = Board::new(SCREEN_WIDTH, SCREEN_HEIGHT, cell_size);
        let solver = djikstra::Solver::from(&board);
        let generator: Box<dyn Generator> = match SELECTED_GENERATOR {
            0 => Box::new(Backtracking::new()),
            1 => Box::new(Kruskal::new(&board)),
            2 => Box::new(Eller::new(&board)),
            _ => panic!(),
        };
        STATE = State::Wait;
        (board, generator, solver)
    }
}

#[allow(static_mut_refs)]
fn main() {
    unsafe {
        // initialize the maze
        let mut cell_size = CELL_SIZE;

        let mut text_buffer: Vec<u8> = vec![20; 0];
        text_buffer.extend_from_slice(format!("{}", cell_size).as_bytes());

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

        let (mut board, mut generator, mut solver) = init_maze(CELL_SIZE);

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
                if new_cell_size != board.cell_size {
                    cell_size = new_cell_size;
                    (board, generator, solver) = init_maze(cell_size);
                }
            }

            raylib::DrawText(
                CString::new("Generator: ").expect("cstr").as_ptr(),
                SCREEN_WIDTH - 300,
                110,
                24,
                raylib::WHITE,
            );

            let mut new_generator = SELECTED_GENERATOR;
            raygui::GuiComboBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 300.0,
                    y: 140.0,
                    width: 200.0,
                    height: 30.0,
                },
                str!("recursive backtracker;kruskal;eller"),
                &mut new_generator,
            );
            if new_generator != SELECTED_GENERATOR {
                SELECTED_GENERATOR = new_generator;
                (board, generator, solver) = init_maze(cell_size);
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
                (board, generator, solver) = init_maze(cell_size);
                STATE = State::Generate;
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
                STATE = State::Solve;
            }
            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 100.0,
                    y: 300.0,
                    width: 80.0,
                    height: 30.0,
                },
                CString::new("reset").expect("cstr").as_ptr(),
            ) != 0
            {
                (board, generator, solver) = init_maze(cell_size);
            }

            // draw the board
            board.draw();
            match STATE {
                State::Wait | State::GenerationDone => {}
                State::Generate => STATE = generator.step(&mut board),
                State::Solve => STATE = solver.step(&board),
                State::Done => path::draw_path(&board, &solver),
            }

            raylib::EndDrawing();
        }
        raylib::CloseWindow();
    }
}
