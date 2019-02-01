use rand;

struct LifeGame {
    row: usize,
    column: usize,
    grid: Vec<Vec<bool>>,
}

impl LifeGame {
    fn new(row: usize, column: usize) -> LifeGame {
        let mut grid = vec![vec![false; column]; row];

        for r in 0..row {
            for c in 0..column {
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
        for columns in &self.grid {
            for val in columns {
                if *val {
                    print!("■")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }
}

fn main() {
    let lg = LifeGame::new(10, 10);
    lg.render()
}
