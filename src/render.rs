use raylib::prelude::*;

use crate::game::grid::{Grid, Cell};
use crate::constants::*;

pub fn init() -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .size(WINDOW::WIDTH, WINDOW::HEIGHT)
        .title(WINDOW::TITLE)
        .resizable()
        .build()
}

pub fn draw_cell(drawing: &mut RaylibDrawHandle<'_>, x: i32, y: i32, grid: &Grid, cell: Cell, neighbor_mine_count: usize) {
    let mut color = 
        if cell.is_hidden {
            GRID::CELL_HIDDEN_COLOR
        } else {
            GRID::CELL_REVEALED_COLOR
        };

    if grid.lost && !cell.is_hidden && cell.is_mine {
        color = GRID::CELL_REVEALED_MINE_COLOR
    }

    drawing.draw_rectangle(
        x,
        y,

        GRID::CELL_SIZE,
        GRID::CELL_SIZE,

        color,
    );

    drawing.draw_rectangle_lines(
        x,
        y,

        GRID::CELL_SIZE,
        GRID::CELL_SIZE,

        GRID::CELL_OUTLINE_COLOR,
    );

    if !cell.is_hidden || cell.is_flagged {
        let text =
            if cell.is_flagged {
                "F".to_string()
            } else if !cell.is_hidden && !cell.is_mine {
                if neighbor_mine_count > 0 {
                    neighbor_mine_count.to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

        drawing.draw_text(
            &text,

            x,
            y,

            GRID::CELL_SIZE,

            Color::BLACK,
        );
    }
}

pub fn draw_grid(drawing: &mut RaylibDrawHandle<'_>, grid: &Grid) {
    for y in 0..grid.size_y {
        for x in 0..grid.size_x {
            let index = grid.get_cell_index(x, y);
            let cell = grid.cells[index];

            let grid_x = x as i32 * GRID::CELL_SIZE;
            let grid_y = y as i32 * GRID::CELL_SIZE;

            let neighbor_mine_count = grid.get_neighbor_mine_count(index);

            draw_cell(drawing, grid_x, grid_y, grid, cell, neighbor_mine_count);
        }
    }
}