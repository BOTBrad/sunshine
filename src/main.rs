extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate image;
extern crate opengl_graphics;
extern crate texture;

use glutin_window as gl_window;
use opengl_graphics as gl_gfx;
use piston::event_loop::{self, EventLoop};
use piston::input;
use piston::input::keyboard::Key;
use piston::window;

mod assets;
mod controller;
mod game;
mod hero;
mod logic;
mod world;

const WIDTH: u32 = 20 * 16;
const HEIGHT: u32 = 20 * 16;

fn main() {
  let opengl = gl_gfx::OpenGL::V3_2;
  let mut window: gl_window::GlutinWindow = window::WindowSettings::new("sunshine", [WIDTH, HEIGHT])
    .opengl(opengl)
    .build()
    .unwrap();

  let mut game = game::State::new(gl_gfx::GlGraphics::new(opengl));
  let mut controller = controller::Controller::new(Key::W, Key::A, Key::S, Key::D);

  let mut events = event_loop::Events::new(event_loop::EventSettings::new())
    .max_fps(60)
    .swap_buffers(true);
  while let Some(e) = events.next(&mut window) {
    match e {
      input::Input::Render(r) => {
      game.draw(&r);
    },

      input::Input::Press(input::Button::Keyboard(key)) => {
        controller.down(&key);
      },

      input::Input::Release(input::Button::Keyboard(key)) => {
        controller.up(&key);
      },

      input::Input::Update(_) => {
        game.update(&controller);
      },

      input::Input::Close(_) => {
        break;
      },

      _ => {},
    }
  }
}
