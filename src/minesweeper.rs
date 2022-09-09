use core::fmt;
use rand::Rng;
use std::collections::HashSet;

use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Cell {
    state: CellState,
    value: CellValue,
}

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub enum CellValue {
    Mine,
    MineCount,
}

#[derive(Debug)]
#[wasm_bindgen]
pub enum CellState {
    Revealed,
    Flagged,
    Closed,
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Minesweeper {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Minesweeper {
    pub fn new(width: u32, height: u32, mine_count: u32) -> Minesweeper {
        Minesweeper {
            width,
            height,
            cells: {
                let mut cells = Vec::new();
                let mut mines = HashSet::new();

                for _ in 0..mine_count {
                    let mine_pos = rand::thread_rng().gen_range(0..height * width);
                    mines.insert(mine_pos);
                }

                for x in 0..height * width {
                    let cell_value = if mines.contains(&x) {
                        CellValue::Mine
                    } else {
                        CellValue::MineCount
                    };

                    cells.push(Cell {
                        state: CellState::Closed,
                        value: cell_value,
                    });
                }

                cells
            },
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn cell(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn reveal_cell(&mut self, row: u32, col: u32) {
        if let Some(cell) = self.get_cell_mut(row, col) {
            cell.state = CellState::Revealed;
        }
    }

    pub fn toggle_flag(&mut self, row: u32, column: u32) {
        if let Some(c) = self.get_cell_mut(row, column) {
            match c.state {
                CellState::Closed => c.state = CellState::Flagged,
                CellState::Flagged => c.state = CellState::Closed,
                _ => {}
            }
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_cell(&self, row: u32, column: u32) -> Option<&Cell> {
        self.cells.get(self.get_index(row, column))
    }

    fn get_cell_mut(&mut self, row: u32, column: u32) -> Option<&mut Cell> {
        let idx = self.get_index(row, column);
        self.cells.get_mut(idx)
    }

    fn count_mines(&self, row: u32, col: u32) -> u8 {
        let row_min = if row > 0 { row - 1 } else { row };
        let row_max = if row >= self.width - 1 { row } else { row + 1 };
        let col_min = if col > 0 { col - 1 } else { col };
        let col_max = if col >= self.height - 1 { col } else { col + 1 };

        (row_min..=row_max)
            .flat_map(move |i| (col_min..=col_max).map(move |j| (i, j)))
            .filter(move |&pos| pos != (row, col))
            .fold(0, |acc, (x, y)| {
                if let Some(c) = self.get_cell(x, y) {
                    if c.value == CellValue::Mine {
                        return acc + 1;
                    };
                };
                acc
            })
    }
}

impl fmt::Display for Minesweeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.width {
            for col in 0..self.height {
                let cell = match self.get_cell(row, col) {
                    Some(c) => c,
                    None => return Err(fmt::Error),
                };
                let symbol = match cell.state {
                    CellState::Revealed => match cell.value {
                        CellValue::Mine => "*",
                        CellValue::MineCount => match self.count_mines(row, col) {
                            1 => "1",
                            2 => "2",
                            3 => "3",
                            4 => "4",
                            5 => "5",
                            6 => "6",
                            7 => "7",
                            8 => "8",
                            _ => "0",
                        },
                    },
                    CellState::Flagged => "F",
                    CellState::Closed => "#",
                };
                write!(f, " {} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::minesweeper::Minesweeper;

    #[test]
    fn setup() {
        let ms = Minesweeper::new(4, 4, 5);
        println!("{}", ms);
        println!("{:?}", ms);
    }

    #[test]
    fn test_open_and_flag() {
        let mut ms = Minesweeper::new(4, 4, 5);
        for i in 0..ms.height {
            for j in 0..ms.width {
                if i < 2 {
                    ms.reveal_cell(i, j);
                } else {
                    ms.toggle_flag(i, j);
                }
            }
        }
        println!("{}", ms);
        println!("{:?}", ms);
    }
}
