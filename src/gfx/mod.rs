mod sdl;

pub use self::sdl::*;

use tile;

pub trait Graphics {
  fn clear(&mut self);
  fn draw(&mut self, x: u32, y: u32, tile: tile::Type);
  fn present(&mut self);
}

