use sdl2;

use graphics;
use tile;

pub struct Sdl2Graphics {
  canvas: sdl2::render::WindowCanvas,
  tile_size: u32,
}

impl Sdl2Graphics {
  pub fn new(width: u32, height: u32, tile_size: u32) -> Sdl2Graphics {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
      "rust-sdl2 demo: Video",
      (width + 2) * tile_size,
      (height + 2) * tile_size,
      ).build()
      .unwrap();

    Sdl2Graphics {
      canvas: window.into_canvas().build().unwrap(),
      tile_size: tile_size,
    }
  }
}

impl graphics::Graphics for Sdl2Graphics {
  fn clear(&mut self) {
    self.canvas.clear();
  }

  fn draw(&mut self, x: u32, y: u32, tile: tile::Type) {
    if tile == tile::Type::Floor {
      return;
    }

    self.canvas.fill_rect(sdl2::rect::Rect::new(
      (x * self.tile_size) as i32,
      (y * self.tile_size) as i32,
      self.tile_size,
      self.tile_size,
    )).unwrap();
  }

  fn present(&mut self) {
    self.canvas.present();
  }
}
