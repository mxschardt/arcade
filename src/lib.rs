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
            width, // 0..width
            height,
            mines: {
                let mut mines = HashSet::new();
                while mines.len() < mine_count {
                    let rand_width = rand::thread_rng().gen_range(0..=width);
                    let rand_height = rand::thread_rng().gen_range(0..=height);

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
        let x_max = if x < self.height { x + 1 } else { x };
        let y_min = if y > 0 { y - 1 } else { y };
        let y_max = if y < self.width { y + 1 } else { y };

        (x_min..x_max)
            .flat_map(move |i| (y_min..y_max).map(move |j| (i, j)))
            .filter(|&(i, j)| i != j)
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
}

#[cfg(test)]
mod tests {
    use crate::{Minesweeper, RevealResult};

    #[test]
    fn test() {
        let ms = Minesweeper::new(10, 10, 5);

        println!("{:?}", ms);
    }

    #[test]
    fn test_counting_mines() {
        let empty_ms = Minesweeper::new(10, 10, 0);
        assert_eq!(0, empty_ms.count_mines((1, 1)));

        let mut full_ms = Minesweeper::new(2, 2, 9);
        match full_ms.reveal_cell((1, 1)) {
            RevealResult::Mine => {}
            RevealResult::MineCount(c) => {
                panic!("The Minefield is not full!");
            }
        }
    }
}
