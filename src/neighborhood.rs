pub struct Neighborhood {
    rx: (usize, usize),
    x: (usize, usize),
    dx: (isize, isize),
}

impl Neighborhood {
    pub fn new(x: (usize, usize), rx: (usize, usize)) -> Self {
        Self {
            x,
            rx,
            dx: (-1, -1),
        }
    }
}

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


/*
#[test]
fn test_neighbor() {
    let nr = 4;
    let nc = 5;
    let ir = 3;
    let ic = 4;
    
    let field: Vec<Vec<char>> = 
        (0..nr).map(|r| {
            (0..nc).map(|c| {
                char::from((b'A' as usize + r * nc + c) as u8)
            })
            .collect()
        })
        .collect();
    
    for i in 0..nr {
        for j in 0..nc {
            if i == ir && j == ic {
                print!("*");
            } else {
                print!("{}", field[i][j]);
            }
        }
        println!();
    }
    println!();
        
    for (r, c) in Neighborhood::new((ir, ic), (nr, nc)) {
        print!("{}", field[r][c]);
    }
    println!();
}
*/
