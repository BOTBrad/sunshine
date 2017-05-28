use std::io;

use graphics;
use opengl_graphics as gl_gfx;
use piston::input;

use board;
use hero;
use tile;

pub struct State {
  board: board::Board,
  hero: hero::Hero,
  gl: gl_gfx::GlGraphics,
}

impl State {
  pub fn new(width: u32, height: u32, gl: gl_gfx::GlGraphics) -> State {
    State {
      board: board::Board {
        width: width,
        height: height,
      },
      hero: hero::Hero {
        pos: ((width as f64) / 2.0, (height as f64) / 2.0),
      },
      gl: gl,
    }
  }

  pub fn draw(&mut self, args: &input::RenderArgs) {
    let board = &self.board;
    let hero = &self.hero;

    self.gl.draw(args.viewport(), |c, g| {
      graphics::clear([0.0, 0.0, 0.0, 1.0], g);
      let (x, y) = hero.pos;
      graphics::rectangle(
        [1.0, 1.0, 1.0, 1.0],
        [x * 16.0, y * 16.0, 16.0, 16.0],
        c.transform, g,
      );
    });

  }

  pub fn update(&mut self, dir: hero::Dir) {
    self.hero.walk(dir);
  }
}
