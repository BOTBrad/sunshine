extern crate sdl2;

mod game;
mod board;
mod hero;

fn main() {
  let mut game = game::State::new();

  loop {
    game.draw();
    let exit = game.update();

    if exit {
      break;
    }
  }
}
