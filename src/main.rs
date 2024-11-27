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

use ggez::{self, error, event, graphics};

// Code adapted from the `ggez` home page.
// https://ggez.rs

// Code inspired by
// http://github.com/ironwall/rust-conway_life_game

struct MainState {
    world: World,
    cell_size: f32,
    world_size: (usize, usize),
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let world_size = (300, 200);
        let cell_size = 2.0;
        let s = MainState {
            world: World::random((world_size.1, world_size.0)),
            cell_size,
            world_size,
        };
        Ok(s)
    }
}

impl event::EventHandler<error::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.world.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let cs = self.cell_size;
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [0.5 * cs, 0.5 * cs],
            0.6 * cs,
            0.25,
            graphics::Color::WHITE,
        )?;

        for (r, c, alive) in self.world.cells() {
            if alive {
                let coord = [c as f32 * cs, r as f32 * cs];
                canvas.draw(&circle, coord);
            }
        }

        canvas.finish(ctx)
    }

    // Adapted from `ggez/examples/graphics_settings.rs`.
    fn resize_event(
        &mut self,
        _ctx: &mut ggez::Context,
        width: f32,
        height: f32,
    ) -> ggez::GameResult {
        self.cell_size = f32::min(
            width / self.world_size.0 as f32,
            height / self.world_size.1 as f32,
        );
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let state = MainState::new()?;

    let cb =
        ggez::ContextBuilder::new("Life", "PSU CS 410/510 Rust Su2019");
    let setup = ggez::conf::WindowSetup::default();
    let mode = ggez::conf::WindowMode::default()
        .dimensions(
            state.world_size.0 as f32 * state.cell_size,
            state.world_size.1 as f32 * state.cell_size,
        )
        .resizable(true);

    let (ctx, event_loop) = cb
        .window_setup(setup.title("Life"))
        .window_mode(mode)
        .build()?;

    event::run(ctx, event_loop, state)
}
