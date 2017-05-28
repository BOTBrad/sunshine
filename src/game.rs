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
        pos: ((width as i32)/2, (height as i32)/2),
      },
      gl: gl,
    }
  }

  pub fn draw(&mut self, args: &input::RenderArgs) {
    let board = &self.board;
    let hero = &self.hero;

    self.gl.draw(args.viewport(), |c, g| {
      graphics::clear([0.0, 0.0, 0.0, 1.0], g);
      for y in 0..board.height {
        for x in 0..board.width {
          if (x as i32, y as i32) == hero.pos {
            graphics::rectangle(
              [1.0, 1.0, 1.0, 1.0],
              [(x as f64) * 16.0, (y as f64) * 16.0, 16.0, 16.0],
              c.transform,
              g,
            );
          }
        }
      }
    });

  }

  pub fn update(&mut self, dir: hero::Dir) {
    self.hero.walk(dir);
  }
}
