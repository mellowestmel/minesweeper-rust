use raylib::prelude::Color;

pub struct WINDOW;
impl WINDOW {
    pub const TITLE: &str = "minesweeper-rust";

    pub const HEIGHT: i32 = 600;
    pub const WIDTH: i32 = 800;
}

pub struct GRID;
impl GRID {
    pub const CELL_REVEALED_MINE_COLOR: Color = Color::RED;

    pub const CELL_REVEALED_COLOR: Color = Color::LIGHTGRAY;
    pub const CELL_HIDDEN_COLOR: Color = Color::GRAY;

    pub const CELL_OUTLINE_COLOR: Color = Color::BLACK;

    pub const CELL_SIZE: i32 = 32;
}