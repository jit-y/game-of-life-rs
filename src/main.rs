use failure;
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
        print!("\x1b[{};{}H", 0, 0);
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
                new_grid[r][c] = self.current_state(r, c)
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
            + self.count(current_row + 1, current_column - 1)
            + self.count(current_row + 1, current_column)
            + self.count(current_row + 1, current_column + 1);

        match val {
            2 => self.grid[r][c],
            3 => true,
            _ => false,
        }
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
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err.as_fail());
            1
        }
    });
}

fn run() -> Result<(), failure::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        return Err(failure::err_msg(format!(
            "Usage: {} <row> <colum>",
            args[0]
        )));
    }

    let row = args[1].parse::<usize>()?;
    let column = args[2].parse::<usize>()?;

    let mut lg = LifeGame::new(row, column);
    let sleep_sec = time::Duration::new(1, 0);
    print!("\x1b[2J");
    lg.render();

    loop {
        thread::sleep(sleep_sec);
        lg.update();
        lg.render();
    }
}
