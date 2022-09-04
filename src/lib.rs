use std::collections::HashSet;
use rand::Rng;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    mines: HashSet<Position>,
    open_cells: HashSet<Position>,
    flagged_cells: HashSet<Position>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
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
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let ms = Minesweeper::new(10, 10, 5);

        println!("{:?}", ms);
    }
}
