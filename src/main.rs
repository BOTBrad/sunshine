extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use glutin_window as gl_window;
use opengl_graphics as gl_gfx;
use piston::event_loop::{self, EventLoop};
use piston::input;
use piston::window;

mod game;
mod board;
mod hero;
mod tile;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;
const TILE_SIZE: u32 = 16;

fn main() {
  let opengl = gl_gfx::OpenGL::V3_2;
  let mut window: gl_window::GlutinWindow = window::WindowSettings::new("sunshine", [WIDTH*TILE_SIZE, HEIGHT*TILE_SIZE])
    .opengl(opengl)
    .build()
    .unwrap();

  let mut game = game::State::new(WIDTH, HEIGHT, gl_gfx::GlGraphics::new(opengl));

  let mut events = event_loop::Events::new(event_loop::EventSettings::new())
    .max_fps(60)
    .swap_buffers(true);
  while let Some(e) = events.next(&mut window) {
    match e {
      input::Input::Render(r) => {
      game.draw(&r);
    },

      input::Input::Update(_) => {
        let exit = game.update();

        if exit {
          break;
        }
      },

      input::Input::Close(_) => {
        break;
      },

      _ => {},
    }
  }
}
