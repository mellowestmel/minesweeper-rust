use raylib::prelude::*;
use rand::seq::SliceRandom;

use crate::constants::GRID;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub is_mine: bool,
    pub state: CellState,
}

pub struct Grid {
    pub lost: bool,

    pub size_x: usize,
    pub size_y: usize,
    
    pub mine_count: usize,

    pub cells: Vec<Cell>,
}

impl Grid {
    // Create grid
    pub fn create(size_x: usize, size_y: usize, mine_count: usize) -> Self {
        let area = size_x * size_y;

        assert!(mine_count <= area);

        let mut cells = vec![
            Cell {
                is_mine: false,
                state: CellState::Hidden,
            };
            area
        ];

        let mut mine_positions: Vec<usize> = (0..area).collect();
        mine_positions.shuffle(&mut rand::rng());

        for position in mine_positions.iter().take(mine_count) {
            cells[*position].is_mine = true;
        }

        Self {
            lost: false,

            size_x,
            size_y,

            mine_count,

            cells,
        }
    }

    pub fn step(&mut self, raylib_handle: &RaylibHandle) {
        if !self.lost && let Some(index) = self.get_hovered_cell(raylib_handle) {
            if raylib_handle.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                self.reveal_cell(index);
            } else if raylib_handle.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT) {
                self.flag_cell(index);
            }
        }
    }

    pub fn get_hovered_cell(&self, raylib_handle: &RaylibHandle) -> Option<usize> {
        let mouse_position = raylib_handle.get_mouse_position();

        let mouse_x = mouse_position.x as i32;
        let mouse_y = mouse_position.y as i32;

        let grid_x = mouse_x / GRID::CELL_SIZE;
        let grid_y = mouse_y / GRID::CELL_SIZE;

        if grid_x < 0 || grid_y < 0 {
            return None;
        }

        if grid_x >= self.size_x as i32 || grid_y >= self.size_y as i32 {
            return None;
        }

        let index = self.get_cell_index(grid_x as usize, grid_y as usize);
        Some(index)
    }

    pub fn reveal_mines(&mut self) {
        for index in 0..self.cells.len() {
            if self.cells[index].is_mine {
                self.cells[index].state = CellState::Revealed;
            }
        }
    }

    pub fn reveal_neighbors(&mut self, index: usize) {
        for neighbor_index in self.get_neighbors(index) {
            if self.cells[neighbor_index].state == CellState::Hidden {
                self.reveal_cell(neighbor_index);
            }
        }
    }

    pub fn reveal_cell(&mut self, index: usize) {
        if self.lost || index >= self.cells.len() {
            return;
        }

        let state = self.cells[index].state;

        // Revealing a hidden cell
        if state == CellState::Hidden {
            self.cells[index].state = CellState::Revealed;

            if self.cells[index].is_mine {
                self.lost = true;
                self.reveal_mines();
                return;
            }

            if self.get_neighbor_mine_count(index) == 0 {
                self.reveal_neighbors(index)
            }
        }

        // Chording
        else if state == CellState::Revealed {
            let neighbors = self.get_neighbors(index);
            let mine_count = self.get_neighbor_mine_count(index);

            let flagged_count = neighbors
                .iter()
                .filter(|&&i| self.cells[i].state == CellState::Flagged)
                .count();

            if mine_count > 0 && flagged_count == mine_count {
                self.reveal_neighbors(index)
            }
        }
    }

    pub fn flag_cell(&mut self, index: usize) {
        let cell = &mut self.cells[index];

        if cell.state == CellState::Hidden {
            cell.state = CellState::Flagged;
        } else if cell.state == CellState::Flagged {
            cell.state = CellState::Hidden;
        }
    }

    pub fn get_neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let (x, y) = self.get_cell_position(index);

        for offset_y in -1..=1 {
            for offset_x in -1..=1 {
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }

                let neighbor_x = x as i32 + offset_x;
                let neighbor_y = y as i32 + offset_y;

                if neighbor_x < 0 || neighbor_y < 0
                    || neighbor_x >= self.size_x as i32
                    || neighbor_y >= self.size_y as i32
                {
                    continue;
                }

                let neighbor_index = self.get_cell_index(
                    neighbor_x as usize,
                    neighbor_y as usize,
                );

                neighbors.push(neighbor_index);
            }
        }

        neighbors
    }

    pub fn get_neighbor_mine_count(&self, index: usize) -> usize {
        self.get_neighbors(index)
            .iter()
            .filter(|&&neighbor_index| self.cells[neighbor_index].is_mine)
            .count()
    }


    pub fn get_cell_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size_x
    }

    pub fn get_cell_position(&self, index: usize) -> (usize, usize) {
        let x = index % self.size_x;
        let y = index / self.size_x;

        (x, y)
    }
}