use raylib::prelude::*;

mod constants;
mod render;
mod game;

fn main() {
    let (mut raylib_handle, raylib_thread) = render::init();
    let mut current_grid = game::grid::Grid::create(15, 10, 15);

    while !raylib_handle.window_should_close() {
        current_grid.step(&raylib_handle);

        let mut drawing = raylib_handle.begin_drawing(&raylib_thread);
        drawing.clear_background(Color::WHITE);

        render::draw_grid(&mut drawing, &current_grid);
    }
}