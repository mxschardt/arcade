use core::fmt;
use rand::Rng;
use std::collections::HashSet;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    mines: HashSet<Position>,
    open_cells: HashSet<Position>,
    flagged_cells: HashSet<Position>,
}

pub enum RevealResult {
    Mine,
    MineCount(u8),
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            mines: {
                let mut mines = HashSet::new();
                while mines.len() < mine_count {
                    let rand_width = rand::thread_rng().gen_range(0..width);
                    let rand_height = rand::thread_rng().gen_range(0..height);

                    mines.insert((rand_width, rand_height));
                }

                mines
            },
            open_cells: HashSet::new(),
            flagged_cells: HashSet::new(),
        }
    }

    pub fn reveal_cell(&mut self, p: Position) -> RevealResult {
        self.open_cells.insert(p);

        let is_mine = self.mines.contains(&p);

        if is_mine {
            RevealResult::Mine
        } else {
            RevealResult::MineCount(self.count_mines(p))
        }
    }

    fn get_neighbors_pos(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let x_min = if x > 0 { x - 1 } else { x };
        let x_max = if x >= self.width { x } else { x + 2 };
        let y_min = if y > 0 { y - 1 } else { y };
        let y_max = if y >= self.height { y } else { y + 2 };

        (x_min..x_max)
            .flat_map(move |i| (y_min..y_max).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }
    // Only non-mines positions expected
    fn count_mines(&self, p: Position) -> u8 {
        self.get_neighbors_pos(p).fold(0, |acc, item| {
            if self.mines.contains(&item) {
                acc + 1
            } else {
                acc
            }
        })
    }

    fn flag_cell(&mut self, p: Position) {
        if !self.flagged_cells.insert(p) {
            self.flagged_cells.remove(&p);
        };
    }

    fn format_cell(&self, p: Position) -> String {
        if self.open_cells.contains(&p) {
            if self.mines.contains(&p) {
                format!("*")
            } else {
                format!("{}", self.count_mines(p))
            }
        } else if self.flagged_cells.contains(&p) {
            format!("f")
        } else {
            format!("#")
        }
    }
}

impl fmt::Display for Minesweeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, " {} ", self.format_cell((i, j)))?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Minesweeper, RevealResult};

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
                    ms.reveal_cell((i, j));
                } else {
                    ms.flag_cell((i, j));
                }
            }
        }
        println!("{}", ms);
        println!("{:?}", ms);
    }

    #[test]
    fn test_neighbour_pos() {
        let ms = Minesweeper::new(2, 2, 0);

        assert_eq!(3, ms.get_neighbors_pos((0, 0)).count());
        assert_eq!(5, ms.get_neighbors_pos((0, 1)).count());
        assert_eq!(8, ms.get_neighbors_pos((1, 1)).count());
    }
    #[test]
    fn test_counting_mines() {
        let empty_ms = Minesweeper::new(10, 10, 0);
        assert_eq!(0, empty_ms.count_mines((1, 1)));

        let mut full_ms = Minesweeper::new(2, 2, 9);
        match full_ms.reveal_cell((1, 1)) {
            RevealResult::Mine => {}
            RevealResult::MineCount(_c) => {
                panic!("The Minefield is not full!");
            }
        }
    }

    #[test]
    fn test_flags() {
        let mut ms = Minesweeper::new(10, 10, 0);

        ms.flag_cell((1, 1));
        assert_eq!(ms.flagged_cells.len(), 1);
        ms.flag_cell((1, 1));
        assert_eq!(ms.flagged_cells.len(), 0);
    }
}
