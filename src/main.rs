use std::fmt::*;

use ndarray::prelude::*;
use rand::*;

const DIMS: (usize, usize) = (20, 70);

struct World(Array2<bool>);

impl World {
    fn new() -> Self {
        World(Array2::default(DIMS))
    }

    fn random<T: Rng>(rng: &mut T) -> Self {
        let mut world = Self::new();
        for cell in world.0.iter_mut() {
            *cell = rng.gen();
        }
        world
    }

}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for r in 0..DIMS.0 {
            for c in 0..DIMS.1 {
                if self.0[[r, c]] {
                    write!(f, "●")?;
                } else {
                    write!(f, "·")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut rng = thread_rng();
    let w = World::random(&mut rng);
    print!("{}", w);
}
