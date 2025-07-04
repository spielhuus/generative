use generative::{
    maze::{
        Board, Generator, State,
        djikstra::{self, Solver},
        generator::{
            aldous_broder::AldousBroder, backtracking::Backtracking, binary_tree::BinaryTree,
            eller::Eller, growing_tree::GrowingTree, hunt_and_kill::HuntAndKill, kruskal::Kruskal,
            prim::Prim, recursive_division::RecursiveDivision, sidewinder::Sidewinder,
            wilson::Wilson,
        },
        path,
    },
    raygui, raylib, str,
};

use std::ffi::CString;

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 800;
const TITLE: &str = "";
const BORDER: i32 = 5;

static mut STATE: State = State::Wait;
static mut SELECTED_GENERATOR: i32 = 0;
static mut SELECTED_SOLVER: i32 = 0;
static mut STEP: i32 = 0;

fn init_maze(cell_count: i32, cell_size: i32) -> (Board, Box<dyn Generator>, Solver) {
    unsafe {
        let mut board = Board::new(BORDER, cell_count, cell_size);
        let solver = djikstra::Solver::from(&board);
        let generator: Box<dyn Generator> = match SELECTED_GENERATOR {
            0 => Box::new(Backtracking::new()),
            1 => Box::new(Kruskal::new(&board)),
            2 => Box::new(Eller::new(&board)),
            3 => Box::new(Prim::new(&board)),
            4 => Box::new(RecursiveDivision::new(&mut board)),
            5 => Box::new(AldousBroder::new(&board)),
            6 => Box::new(Wilson::new(&board)),
            7 => Box::new(HuntAndKill::new(&board)),
            8 => Box::new(GrowingTree::new(&board)),
            9 => Box::new(BinaryTree::new()),
            10 => Box::new(Sidewinder::new(&mut board)),
            _ => panic!(),
        };
        STATE = State::Wait;
        STEP = 0;

        (board, generator, solver)
    }
}

#[allow(static_mut_refs)]
fn main() {
    unsafe {
        // initialize the maze
        let mut cell_count = 5;
        let mut cell_size = (SCREEN_HEIGHT - 2 * BORDER) / cell_count;
        let mut step_by_step = false;
        let mut step = false;

        let mut slider = 0;

        let mut text_buffer: Vec<u8> = vec![20; 0];
        text_buffer.extend_from_slice(format!("   {}", cell_count).as_bytes());

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

        let (mut board, mut generator, mut solver) = init_maze(cell_count, cell_size);

        // main loop
        while !raylib::WindowShouldClose() {
            raylib::BeginDrawing();
            raylib::ClearBackground(raylib::BLACK);

            // draw the ui
            raylib::DrawText(
                CString::new("Size: ").expect("cstr").as_ptr(),
                SCREEN_WIDTH - 350,
                70,
                24,
                raylib::WHITE,
            );

            let mut new_slider = slider;
            raygui::GuiComboBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 110.0,
                    width: 300.0,
                    height: 30.0,
                },
                str!("5x5;10x10;20x20;30x30;40x40;50x50;60x60;70x70;80x80;90x90;100x100"),
                &mut new_slider,
            );
            if new_slider != slider {
                let new_count = match new_slider {
                    0 => 5,
                    1 => 10,
                    2 => 20,
                    3 => 30,
                    4 => 40,
                    5 => 50,
                    6 => 60,
                    7 => 70,
                    8 => 80,
                    9 => 90,
                    _ => 100,
                };
                let new_cell_size = (SCREEN_HEIGHT - 2 * BORDER) / new_count;
                slider = new_slider;
                cell_count = new_count;
                cell_size = new_cell_size;
                (board, generator, solver) = init_maze(cell_count, cell_size);
            }

            raylib::DrawText(
                CString::new("Generator: ").expect("cstr").as_ptr(),
                SCREEN_WIDTH - 350,
                160,
                24,
                raylib::WHITE,
            );

            let mut new_generator = SELECTED_GENERATOR;
            raygui::GuiComboBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 190.0,
                    width: 300.0,
                    height: 30.0,
                },
                str!(
                    "recursive backtracker;kruskal;eller;prim;recursive division;aldous broder;wilson;hunt and kill;growing tree;binary tree;sidewinder"
                ),
                &mut new_generator,
            );
            if new_generator != SELECTED_GENERATOR {
                SELECTED_GENERATOR = new_generator;
                (board, generator, solver) = init_maze(cell_count, cell_size);
            }

            raylib::DrawText(
                CString::new("Solver: ").expect("cstr").as_ptr(),
                SCREEN_WIDTH - 350,
                230,
                24,
                raylib::WHITE,
            );

            let mut new_solver = SELECTED_SOLVER;
            raygui::GuiComboBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 270.0,
                    width: 300.0,
                    height: 30.0,
                },
                str!("djikstra;recursive backtracker"),
                &mut new_solver,
            );
            if new_solver != SELECTED_SOLVER {
                SELECTED_SOLVER = new_solver;
                (board, generator, solver) = init_maze(cell_count, cell_size);
            }

            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 400.0,
                    y: 300.0,
                    width: 80.0,
                    height: 30.0,
                },
                CString::new("generate").expect("cstr").as_ptr(),
            ) != 0
            {
                step_by_step = false;
                STATE = State::Generate;
            }
            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 300.0,
                    y: 300.0,
                    width: 80.0,
                    height: 30.0,
                },
                CString::new("step").expect("cstr").as_ptr(),
            ) != 0
            {
                println!("step");
                STATE = State::Generate;
                step_by_step = true;
                step = true;
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
                (board, generator, solver) = init_maze(cell_count, cell_size);
            }

            raylib::DrawText(
                CString::new(format!("Size: {}x{}", cell_count, cell_count))
                    .expect("cstr")
                    .as_ptr(),
                SCREEN_WIDTH - 300,
                400,
                24,
                raylib::WHITE,
            );

            raylib::DrawText(
                CString::new(format!("Step: {}", STEP))
                    .expect("cstr")
                    .as_ptr(),
                SCREEN_WIDTH - 300,
                430,
                24,
                raylib::WHITE,
            );

            // draw the board
            board.draw();

            match STATE {
                State::Wait | State::GenerationDone => {}
                State::Generate => {
                    generator.draw(&board);
                    if !step_by_step || step {
                        STATE = generator.step(&mut board);
                        STEP += 1;
                        step = false;
                    }
                }
                State::Solve => STATE = solver.step(&board),
                State::Done => path::draw_path(&board, &solver),
            }

            raylib::EndDrawing();
        }
        raylib::CloseWindow();
    }
}
