// This program is licensed under the "MIT License". Please
// see the file `LICENSE` in this distribution for license
// terms.

//! Life cell neighborhood iterator.

/// Iterator producing row-column coordinates in the 3x3
/// neighborhood surrounding the given coordinate.
pub struct Neighborhood {
    rx: (usize, usize),
    x: (usize, usize),
    dx: (isize, isize),
}

impl Neighborhood {
    /// Make a new neighborhood centered at `x`. The
    /// returned coordinates will be clipped against the
    /// bounding box `(0, 0)..rx`.
    pub fn new(x: (usize, usize), rx: (usize, usize)) -> Self {
        Self { x, rx, dx: (-1, -1) }
    }
}

// Check coordinate `x` for in-range `0..rx` after offset
// `dx` and return the resulting offset coordinate if any.
fn clip(x: usize, rx: usize, dx: isize) -> Option<usize> {
    let x = x as isize;
    let rx = rx as isize;
    let nx = x + dx;
    if nx >= 0 && nx < rx {
        Some(nx as usize)
    } else {
        None
    }
}

impl Iterator for Neighborhood {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.dx.0 <= 1 {
            while self.dx.1 <= 1 {
                let r = clip(self.x.0, self.rx.0, self.dx.0);
                let c = clip(self.x.1, self.rx.1, self.dx.1);
                self.dx.1 += 1;
                if let (Some(r), Some(c)) = (r, c) {
                    if (r, c) != self.x {
                        return Some((r, c));
                    }
                }
            }
            self.dx.1 = -1;
            self.dx.0 += 1;
        }
        None
    }
}
