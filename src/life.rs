// This program is licensed under the "MIT License". Please
// see the file `LICENSE` in this distribution for license
// terms.

//! Conway's
//! [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
//! as implemented in Portland State University CS410/510
//! Rust Summer 2019.
use crate::neighborhood::*;

use std::fmt::{self, *};

use ndarray::prelude::*;
use rand::prelude::*;

/// Life array / arena / world.
#[derive(Clone)]
pub struct World(Array2<bool>);

impl World {
    /// Make a new empty arena.
    pub fn new(dims: (usize, usize)) -> Self {
        World(Array2::default(dims))
    }

    /// Make a new uniformly-populated arena.
    pub fn random(dims: (usize, usize)) -> Self {
        let mut rng = thread_rng();
        let mut world = Self::new(dims);
        for cell in world.0.iter_mut() {
            *cell = rng.gen();
        }
        world
    }

    /// Update the world in-place according to the Game of
    /// Life rules.
    pub fn update(&mut self) {
        let old = &self.0;
        let mut new = old.clone();
        let dim = self.0.dim();
        for (r, row) in old.outer_iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let count: u8 = Neighborhood::new((r, c), dim)
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

    pub fn cells<'a>(&'a self) -> impl Iterator<Item=(usize, usize, bool)> + 'a {
        self.0.indexed_iter().map(|((r, c), &b)| (r, c, b))
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
        let display_chars = ['·', '●'];
        for row in self.0.axis_iter(Axis(0)) {
            let chars: String = row
                .iter()
                .map(|&on| display_chars[on as usize])
                .collect();
            writeln!(f, "{}", chars)?;
        }
        Ok(())
    }
}

