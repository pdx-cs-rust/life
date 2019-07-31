// This program is licensed under the "MIT License". Please
// see the file `LICENSE` in this distribution for license
// terms.

//! Conway's
//! [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
//! as implemented in Portland State University CS410/510
//! Rust Summer 2019.

mod life;
mod neighborhood;

use life::*;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

// Code adapted from the `ggez` home page.
// https://ggez.rs

// Code inspired by
// http://github.com/ironwall/rust-conway_life_game

const WORLD_SIZE: (usize, usize) = (200, 300);
const CELL_SIZE: f32 = 2.0;
const SCREEN_SIZE: (f32, f32) = (
    WORLD_SIZE.1 as f32 * CELL_SIZE,
    WORLD_SIZE.0 as f32 * CELL_SIZE,
);

struct MainState {
    world: World,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { world: World::random(WORLD_SIZE) };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.world.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            0.6 * CELL_SIZE,
            0.5,
            graphics::WHITE,
        )?;

        for (r, c, alive) in self.world.cells() {
            if alive {
                let coord = na::Point2::new(
                    c as f32 * CELL_SIZE,
                    r as f32 * CELL_SIZE,
                );
                graphics::draw(ctx, &circle, (coord,))?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new(
        "Life",
        "PSU CS 410/510 Rust Su2019",
    );
    let setup = ggez::conf::WindowSetup::default();
    let mode = ggez::conf::WindowMode::default();

    let (ctx, event_loop) = &mut cb
        .window_setup(setup.title("Life"))
        .window_mode(mode.dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
