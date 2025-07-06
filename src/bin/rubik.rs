// use generative::raylib;

fn main() {
    // unsafe {
    //     raylib::InitWindow(
    //         SCREEN_WIDTH,
    //         SCREEN_HEIGHT,
    //         CString::new(TITLE).expect("cstr").as_ptr(),
    //     );
    //
    //     let screen_width = raylib::GetScreenWidth() as f32;
    //     let screen_height = raylib::GetScreenHeight() as f32;
    //
    //     let target = raylib::Vector2::new(screen_width - 10.0, screen_height / 2.0);
    //     let mut population = Population::new(target);
    //
    //     raylib::SetTargetFPS(60);
    //
    //     let mut loops = 0;
    //     let mut round = 0;
    //
    //     // Main game loop
    //     while !raylib::WindowShouldClose() {
    //         raylib::BeginDrawing();
    //         raylib::ClearBackground(raylib::BLACK);
    //
    //         let winners = population.winners();
    //         raylib::DrawText(
    //             CString::new(std::format!(
    //                 "Round: {}, Steps: {}, Winners: {}",
    //                 loops,
    //                 round,
    //                 winners
    //             ))
    //             .expect("cstr")
    //             .as_ptr(),
    //             10,
    //             10,
    //             20,
    //             raylib::WHITE,
    //         );
    //         raylib::DrawRectangleRec(WALL_A, raylib::WHITE);
    //         raylib::DrawRectangleRec(WALL_B, raylib::WHITE);
    //         raylib::DrawCircleV(target, 4.0, raylib::GREEN);
    //         population.draw(round);
    //         raylib::EndDrawing();
    //
    //         if round < ROUNDS - 1 {
    //             population.step(round);
    //             round += 1;
    //         } else {
    //             population.fitness();
    //             population.generate();
    //             population.mutate();
    //             round = 0;
    //             loops += 1;
    //         }
    //     }
    //     raylib::CloseWindow();
    // }
}
