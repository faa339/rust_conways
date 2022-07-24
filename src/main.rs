#![allow(dead_code)]

use clap::Parser;
use std::{fmt::Display, fs::File, thread, time};
struct Board {
    old: Vec<usize>,
    new: Vec<usize>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        // We create a buffer zone of just 0's around the board for easier
        // neighbor counts
        Board {
            old: vec![0; (rows + 1) * (cols + 1)],
            new: vec![0; (rows + 1) * (cols + 1)],
            width: cols + 1,
            height: rows + 1,
        }
    }
    fn configure(&mut self, file: Option<File>) -> Result<(), std::io::Error> {
        // Used on first run or when you want a nice reset
        if let Some(f) = file {
            println!(
                "Received config file {:?}; attempting to parse into board",
                f
            );
        }
        println!("No config file received; generating random board");
        for w in 1..self.width - 1 {
            for h in 1..self.height - 1 {
                if rand::random::<usize>() % 100 > 50 {
                    // println!("Setting new at {} {}", w, h);
                    self.set_new(w, h, 1);
                }
            }
        }
        self.copy_board();
        Ok(())
    }
    fn get_old(&self, row: usize, col: usize) -> Option<usize> {
        // retrieve an index from old board
        if row > self.height || col > self.width {
            None
        } else {
            Some(*self.old.get(self.width * row + col).unwrap())
        }
    }
    fn set_new(&mut self, row: usize, col: usize, val: usize) {
        // set state in new board
        if row < self.height && col < self.width {
            *self.new.get_mut(self.width * row + col).unwrap() = val;
        }
    }
    fn copy_board(&mut self) {
        // copy the new to the old
        for (ind, val) in self.new.iter().enumerate() {
            *self.old.get_mut(ind).unwrap() = *val;
        }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        // Count the number of neighbors of a cell
        // Note that this will always check old because thats where we get our new
        // state from
        let mut count = 0;
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if (i, j) == (0, 0) {
                    continue;
                }
                let leftind = (row as i32 + i) as usize;
                let rightind = (col as i32 + j) as usize;
                // println!("{} {}", leftind, rightind);
                count += self.get_old(leftind, rightind).unwrap()
            }
        }
        count
    }

    fn evolve(&mut self) {
        // Actual evolution step of conways
        // Count neighbors of old, assign to new, then copy
        // new to old
        // TODO: FIX THIS
        for w in 1..self.width - 1 {
            for h in 1..self.height - 1 {
                let neighbors = self.count_neighbors(w, h);
                let cell = self.get_old(w, h).unwrap();
                if (cell == 0 && neighbors == 3)
                    || (cell == 1 && (neighbors == 2 || neighbors == 3))
                {
                    self.set_new(w, h, 1);
                } else {
                    self.set_new(w, h, 0);
                }
            }
        }
        self.copy_board();
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = |c: usize| match c {
            1 => '😎',
            _ => '💀',
        };
        for w in 1..self.width {
            for h in 1..self.height {
                let v = self.get_old(w, h).unwrap();
                write!(f, "{} ", x(v))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Parser)]
struct Cli {
    /// Number of generations
    generations: usize,
    /// Width of the board
    width: Option<usize>,
    /// Height of the board
    height: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut b: Board = Board::new(cli.width.unwrap_or(10), cli.height.unwrap_or(10));
    // thread::sleep(time::Duration::from_millis(2000));
    // println!("{} {} {} {}", b.width, b.height, b.old.len(), b.new.len());
    b.configure(None)?;
    for g in 0..cli.generations {
        // print!("{}[2J", 27 as char);
        print!("\x1B[2J");
        println!("Generation:{}", g + 1);
        print!("{}", b);
        thread::sleep(time::Duration::from_millis(1500));
        if !b.old.contains(&1) {
            println!("No more living cells; exiting");
            break;
        }
        b.evolve();
    }
    Ok(())
}
