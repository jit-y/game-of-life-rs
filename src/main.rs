use rand;
use std::{thread, time};

struct LifeGame {
    row: usize,
    column: usize,
    grid: Vec<Vec<bool>>,
}

impl LifeGame {
    fn new(row: usize, column: usize) -> LifeGame {
        let mut grid = vec![vec![false; column]; row];

        for r in 0..row - 1 {
            for c in 0..column - 1 {
                if rand::random::<u32>() % 10 == 0 {
                    grid[r][c] = true
                }
            }
        }

        LifeGame {
            row: row,
            column: column,
            grid: grid,
        }
    }

    fn render(&self) {
        for r in 0..self.row - 1 {
            for c in 0..self.column - 1 {
                if self.grid[r][c] {
                    print!("■")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }

    fn update(&mut self) {
        let mut new_grid = vec![vec![false; self.column]; self.row];

        for r in 0..self.row - 1 {
            for c in 0..self.column - 1 {
                let current = self.current_state(r, c);
                new_grid[r][c] = current
            }
        }

        self.grid = new_grid;
    }

    fn current_state(&self, r: usize, c: usize) -> bool {
        let current_row = r as i32;
        let current_column = c as i32;

        let val = self.count(current_row - 1, current_column - 1)
            + self.count(current_row - 1, current_column)
            + self.count(current_row - 1, current_column + 1)
            + self.count(current_row, current_column - 1)
            + self.count(current_row, current_column + 1)
            + self.count(current_row - 1, current_column - 1)
            + self.count(current_row - 1, current_column)
            + self.count(current_row - 1, current_column + 1);

        if val == 2 {
            return self.grid[r][c];
        } else if val == 3 {
            return true;
        }

        false
    }

    fn count(&self, r: i32, c: i32) -> u32 {
        if r < 0 || r >= self.row as i32 {
            return 0;
        }

        if c < 0 || c >= self.column as i32 {
            return 0;
        }

        if self.grid[r as usize][c as usize] {
            return 1;
        }

        0
    }
}

fn main() {
    let mut lg = LifeGame::new(25, 50);
    let sleep_sec = time::Duration::new(1, 0);
    lg.render();

    loop {
        thread::sleep(sleep_sec);
        lg.update();
        lg.render();
    }
}
