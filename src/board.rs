use crate::rand::prelude::*;
use std::collections::HashSet;

pub struct Board {
    /// Current state of the board.
    current: [[bool; 32]; 32],

    /// Previous state of the board, for neighbor counting.
    previous: [[bool; 32]; 32],

    /// RNG for randomizing a new board
    rng: ThreadRng,

    /// Keep track of all the boards we've seen before, so we can reset if we get into a loop.
    /// (Note: current hash function could probably be improved, but it seems to work for now.)
    seen_before: HashSet<u64>,

    /// Keep track of the number of generations of this cycle, just for fun.
    generations: u64,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            current: [[false; 32]; 32],
            previous: [[false; 32]; 32],
            rng: rand::thread_rng(),
            seen_before: HashSet::new(),
            generations: 1,
        };
        board.randomize();

        board
    }

    pub fn state(&self, x: usize, y: usize) -> bool {
        self.current[y][x]
    }

    pub fn next(&mut self) {
        for y in 0..32 {
            for x in 0..32 {
                self.previous[y][x] = self.current[y][x];
            }
        }

        for y in 0..32 {
            for x in 0..32 {
                let neighbors = count_neighbors(&self.previous, y as i32, x as i32);

                if self.current[y][x] {
                    // Cell is currently alive.
                    // It stays alive if it has exactly two or three neighbors.
                    // It dies otherwise.
                    if neighbors == 2 || neighbors == 3 {
                        self.current[y][x] = true;
                    }
                    else {
                        self.current[y][x] = false;
                    }
                }
                else {
                    // Cell is currently dead.
                    // It is born if it has exactly three neighbors.
                    // It dies otherwise.
                    if neighbors == 3 {
                        self.current[y][x] = true;
                    }
                    else {
                        self.current[y][x] = false;
                    }
                }
            }
        }

        // Make sure (with *relative* certainty) that we haven't seen this board before.
        // If we have, generate a new random one so we can start the game over.
        let board_hash = self.hash();
        if self.seen_before.contains(&board_hash) {
            println!("Reset after {} generations.", self.generations);
            self.seen_before.clear();
            self.randomize();
            self.generations = 1;
        }
        else {
            self.seen_before.insert(board_hash);
            self.generations += 1;
        }
    }

    fn randomize(&mut self) {
        for y in 0..32 {
            for x in 0..32 {
                self.current[y][x] = self.rng.gen();
            }
        }
    }

    fn hash(&self) -> u64 {
        let mut hash: u64 = 0;

        for row in 1..16 {
            let mut row_hash: u64 = 0;
            // Load upper row into upper bits
            for i in 0..32 {
                row_hash |= if self.current[row*2][i] { 1 << (63 - i) } else { 0 };
            }
            // Load lower row into lower bits
            for j in 0..32 {
                row_hash |= if self.current[row*2+1][j] { 1 << (31 - j) } else { 0 };
            }
            hash ^= row_hash;
        }

        hash
    }
}

fn count_neighbors(board: &[[bool; 32]; 32], y: i32, x: i32) -> u32 {
    let mut neighbors: u32 = 0;

    for i in -1..2 {
        for j in -1..2 {
            let mut index_y = y+i;
            if index_y < 0 {
                index_y += 32;
            }
            if index_y >= 32 {
                index_y -= 32;
            }

            let mut index_x = x+j;
            if index_x < 0 {
                index_x += 32;
            }
            if index_x >= 32 {
                index_x -= 32;
            }

            if !(i==0 && j==0) && board[index_y as usize][index_x as usize] {
                neighbors += 1;
            }
        }
    }

    neighbors
}