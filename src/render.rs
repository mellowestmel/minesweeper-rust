use raylib::prelude::*;

use crate::game::grid::{Grid, Cell, CellState};
use crate::constants::*;

pub fn init() -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .size(WINDOW::WIDTH, WINDOW::HEIGHT)
        .title(WINDOW::TITLE)
        .build()
}

pub fn draw_cell(drawing: &mut RaylibDrawHandle<'_>, x: i32, y: i32, cell: Cell, neighbor_mine_count: usize) {
    let color = 
        if cell.state == CellState::Revealed {
            if cell.is_mine {
                GRID::CELL_REVEALED_MINE_COLOR
            } else {
                GRID::CELL_REVEALED_COLOR
            }
        } else {
            GRID::CELL_HIDDEN_COLOR
        };

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

    if cell.state != CellState::Hidden {
        let text = match cell.state {
            CellState::Revealed => {
                if cell.is_mine {
                    String::new()
                } else {
                    if neighbor_mine_count > 0 {
                        neighbor_mine_count.to_string()
                    } else {
                        String::new()
                    }
                }
            }
            CellState::Flagged => "F".to_string(),
            CellState::Hidden => String::new(),
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

            draw_cell(drawing, grid_x, grid_y, cell, neighbor_mine_count);
        }
    }
}