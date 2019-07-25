// This program is licensed under the "MIT License". Please
// see the file `LICENSE` in this distribution for license
// terms.

//! Conway's
//! [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
//! as implemented in Portland State University CS410/510
//! Rust Summer 2019.

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
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in self.0.axis_iter(Axis(0)) {
            let chars: String = row
                .iter()
                .map(|&on| DISPLAY_CHARS[on as usize])
                .collect();
            writeln!(f, "{}", chars)?;
        }
        Ok(())
    }
}

fn main() {
    let w = World::random();
    print!("{}", w);
}
