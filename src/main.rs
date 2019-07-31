// This program is licensed under the "MIT License". Please
// see the file `LICENSE` in this distribution for license
// terms.

//! Conway's
//! [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
//! as implemented in Portland State University CS410/510
//! Rust Summer 2019.
mod neighborhood;
use neighborhood::*;

use std::fmt::{self, *};

use ndarray::prelude::*;
use rand::prelude::*;

// XXX Hardcoded for now.
/// World dimensions.
const DIMS: (usize, usize) = (20, 70);

// XXX Hardcoded for now.
/// Display characters for empty and full cells.
const DISPLAY_CHARS: [char;2] = ['·', '●'];

/// Life array / arena / world.
#[derive(Clone)]
struct World(Array2<bool>);

impl World {
    /// Make a new empty arena.
    fn new() -> Self {
        World(Array2::default(DIMS))
    }

    /// Make a new uniformly-populated arena.
    fn random() -> Self {
        let mut rng = thread_rng();
        let mut world = Self::new();
        for cell in world.0.iter_mut() {
            *cell = rng.gen();
        }
        world
    }

    /// Update the world in-place according to the Game of
    /// Life rules.
    fn update(&mut self) {
        let old = &self.0;
        let mut new = old.clone();
        for (r, row) in old.outer_iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let count: u8 = Neighborhood::new((r, c), DIMS)
                    .map(|(r, c)| old[[r, c]] as u8)
                    .sum();
                let life = match (cell, count) {
                    (true, count) if count < 2 => false,
                    (true, count) if count <= 3 => true,
                    (true, _) => false,
                    (false, count) if count == 3 => true,
                    _ => false,
                };
                new[[r, c]] = life;
            }
        }
        *self = World(new);
    }
}

impl Iterator for World {
    type Item = World;

    fn next(&mut self) -> Option<Self::Item> {
        self.update();
        Some(self.clone())
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in self.0.outer_iter() {
            let chars: String = row
                .iter()
                .map(|&on| DISPLAY_CHARS[on as usize])
                .collect();
            writeln!(f, "{}", chars)?;
        }
        Ok(())
    }
}

/// Emit an ANSI clear-screen escape sequence.  No flushing
/// is done.
fn clear_screen() {
    print!("\u{1b}[H\u{1b}[2J\u{1b}[3J");
}

/// Run the sim.
fn main() {
    let w = World::random();
    clear_screen();
    print!("{}", w);
    for nw in w {
        let duration =
            std::time::Duration::from_millis(10);
        std::thread::sleep(duration);
        clear_screen();
        print!("{}", nw);
    }
}
