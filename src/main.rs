extern crate sdl2;

mod game;
mod gfx;
mod board;
mod hero;
mod tile;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;
const TILE_SIZE: u32 = 16;

fn main() {
  let mut game = game::State::new(WIDTH, HEIGHT);
  let mut gfx = gfx::Sdl2Graphics::new(
    WIDTH,
    HEIGHT,
    TILE_SIZE,
  );

  loop {
    game.draw(&mut gfx);
    let exit = game.update();

    if exit {
      break;
    }
  }
}
