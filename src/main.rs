extern crate sdl2;

mod game;
mod board;
mod hero;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;

fn main() {
  let mut game = game::State::new(WIDTH, HEIGHT);

  loop {
    game.draw();
    let exit = game.update();

    if exit {
      break;
    }
  }
}
