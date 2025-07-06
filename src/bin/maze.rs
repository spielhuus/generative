use generative::{
    maze::{
        Board, Generator, Solver, State,
        generator::{
            aldous_broder::AldousBroder, backtracking::Backtracking, binary_tree::BinaryTree,
            eller::Eller, growing_tree::GrowingTree, hunt_and_kill::HuntAndKill, kruskal::Kruskal,
            prim::Prim, recursive_division::RecursiveDivision, sidewinder::Sidewinder,
            wilson::Wilson,
        },
        path, solver,
    },
    raygui, raylib, str,
};

use std::ffi::CString;

const SCREEN_WIDTH: usize = 1200;
const SCREEN_HEIGHT: usize = 800;
const TITLE: &str = "";
const BORDER: usize = 5;

static mut STATE: State = State::Wait;
static mut SELECTED_GENERATOR: i32 = 0;
static mut SELECTED_SOLVER: i32 = 0;
static mut STEP: usize = 0;
static mut FAST: bool = false;

fn init_solver(board: &Board) -> Box<dyn Solver> {
    unsafe {
        match SELECTED_SOLVER {
            0 => Box::new(solver::djikstra::Djikstra::new(board)),
            1 => Box::new(solver::backtracker::Backtracker::new(board)),
            2 => Box::new(solver::a_star::AStar::new(board)),
            3 => Box::new(solver::dead_end_filing::DeadEndFilling::new(board)),
            _ => panic!(),
        }
    }
}

fn init_maze(cell_count: usize, cell_size: usize) -> (Board, Box<dyn Generator>, Box<dyn Solver>) {
    unsafe {
        let mut board = Board::new(BORDER, cell_count, cell_size);
        let solver = init_solver(&board);
        let generator: Box<dyn Generator> = match SELECTED_GENERATOR {
            0 => Box::new(Backtracking::new()),
            1 => Box::new(Kruskal::new(&board)),
            2 => Box::new(Eller::new(&board)),
            3 => Box::new(Prim::new(&board)),
            4 => Box::new(RecursiveDivision::new(&mut board)),
            5 => Box::new(AldousBroder::new(&board)),
            6 => Box::new(Wilson::new(&mut board)),
            7 => Box::new(HuntAndKill::new(&mut board)),
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
        let mut error: Option<String> = None;

        let mut slider = 0;

        let mut text_buffer: Vec<u8> = vec![20; 0];
        text_buffer.extend_from_slice(format!("   {}", cell_count).as_bytes());

        raylib::InitWindow(
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32,
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
                (SCREEN_WIDTH - 350) as i32,
                70,
                24,
                raylib::WHITE,
            );

            let mut new_slider = slider;
            raygui::GuiComboBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 100.0,
                    width: 300.0,
                    height: 30.0,
                },
                str!("5x5;10x10;20x20;30x30;40x40;50x50;60x60;70x70;80x80;90x90;100x100;1000x1000"),
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
                    10 => 100,
                    _ => 200,
                };
                let new_cell_size = (SCREEN_HEIGHT - 2 * BORDER) / new_count;
                slider = new_slider;
                cell_count = new_count;
                cell_size = new_cell_size;
                (board, generator, solver) = init_maze(cell_count, cell_size);
            }

            raylib::DrawText(
                CString::new("Generator: ").expect("cstr").as_ptr(),
                (SCREEN_WIDTH - 350) as i32,
                140,
                24,
                raylib::WHITE,
            );

            let mut new_generator = SELECTED_GENERATOR;
            raygui::GuiComboBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 170.0,
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
                (SCREEN_WIDTH - 350) as i32,
                210,
                24,
                raylib::WHITE,
            );

            let mut new_solver = SELECTED_SOLVER;
            raygui::GuiComboBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 240.0,
                    width: 300.0,
                    height: 30.0,
                },
                str!("djikstra;recursive backtracker;a*;dead end filling"),
                &mut new_solver,
            );
            if new_solver != SELECTED_SOLVER {
                SELECTED_SOLVER = new_solver;
                solver = init_solver(&board);
                STATE = State::Wait;
            }

            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 300.0,
                    width: 140.0,
                    height: 30.0,
                },
                CString::new("generate").expect("cstr").as_ptr(),
            ) != 0
            {
                (board, generator, solver) = init_maze(cell_count, cell_size);
                step_by_step = false;
                STATE = State::Generate;
                STEP = 0;
            }
            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 190.0,
                    y: 300.0,
                    width: 140.0,
                    height: 30.0,
                },
                CString::new("solve").expect("cstr").as_ptr(),
            ) != 0
            {
                error = None;
                solver = init_solver(&board);
                STATE = State::Solve;
                STEP = 0;
            }

            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 340.0,
                    width: 140.0,
                    height: 30.0,
                },
                CString::new("step").expect("cstr").as_ptr(),
            ) != 0
            {
                match STATE {
                    State::Wait | State::Generate => {
                        STATE = State::Generate;
                        step_by_step = true;
                        step = true;
                    }
                    State::GenerationDone | State::Solve => {
                        STATE = State::Solve;
                        step_by_step = true;
                        step = true;
                    }
                    _ => {}
                }
            }

            if raygui::GuiButton(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 190.0,
                    y: 340.0,
                    width: 140.0,
                    height: 30.0,
                },
                CString::new("reset").expect("cstr").as_ptr(),
            ) != 0
            {
                (board, generator, solver) = init_maze(cell_count, cell_size);
            }

            raygui::GuiCheckBox(
                raylib::Rectangle {
                    x: SCREEN_WIDTH as f32 - 350.0,
                    y: 380.0,
                    width: 30.0,
                    height: 30.0,
                },
                CString::new("fast").expect("cstr").as_ptr(),
                &FAST,
            );

            // Displaz the state and other info

            raylib::DrawText(
                CString::new(format!("State: {}", STATE))
                    .expect("cstr")
                    .as_ptr(),
                (SCREEN_WIDTH - 350) as i32,
                440,
                24,
                raylib::WHITE,
            );

            raylib::DrawText(
                CString::new(format!("Size: {}x{}", cell_count, cell_count))
                    .expect("cstr")
                    .as_ptr(),
                (SCREEN_WIDTH - 350) as i32,
                470,
                24,
                raylib::WHITE,
            );

            raylib::DrawText(
                CString::new(format!("Step: {}", STEP))
                    .expect("cstr")
                    .as_ptr(),
                (SCREEN_WIDTH - 350) as i32,
                500,
                24,
                raylib::WHITE,
            );

            raylib::DrawText(
                CString::new(format!("Solution length: {}", solver.get_path().len()))
                    .expect("cstr")
                    .as_ptr(),
                (SCREEN_WIDTH - 350) as i32,
                530,
                24,
                raylib::WHITE,
            );

            // draw the board
            board.draw();

            match STATE {
                State::Wait => {}
                State::GenerationDone => {
                    solver = init_solver(&board);
                }
                State::Generate => {
                    generator.draw(&board);
                    if !step_by_step || step {
                        STATE = generator.step(&mut board);
                        STEP += 1;
                        step = false;
                    }
                    while FAST && STATE == State::Generate {
                        if !step_by_step || step {
                            STATE = generator.step(&mut board);
                            STEP += 1;
                            step = false;
                        }
                    }
                }
                State::Solve => {
                    solver.draw(&board);
                    if let Some(error) = &error {
                        raylib::DrawText(
                            CString::new(error.as_str()).expect("cstr").as_ptr(),
                            (SCREEN_WIDTH - 350) as i32,
                            600,
                            24,
                            raylib::RED,
                        );
                    } else if !step_by_step || step {
                        match solver.step(&board) {
                            Ok(state) => {
                                STATE = state;
                                STEP += 1;
                                step = false;
                            }
                            Err(str) => error = Some(str),
                        }

                        while FAST && STATE == State::Solve && error.is_none() {
                            match solver.step(&board) {
                                Ok(state) => {
                                    STATE = state;
                                    STEP += 1;
                                    step = false;
                                }
                                Err(str) => error = Some(str),
                            }
                        }
                    }
                }
                State::Done => path::draw_path(&board, solver.get_path()),
            }

            raylib::EndDrawing();
        }
        raylib::CloseWindow();
    }
}
