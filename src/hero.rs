use image;
use opengl_graphics as gl_gfx;
use texture::TextureSettings;

use crate::assets;
use crate::logic::physics;

pub struct Hero {
  pos: [f64; 2],
  facing: Facing,

  textures: [gl_gfx::Texture; 4],
}

const SPEED: f64 = 1.0/16.0;
const HBOX_SIZE: [f64; 2] = [0.75, 0.9];

impl Hero {
  pub fn new(pos: [f64; 2]) -> Self {
    let east_dimg = image::open(assets::get("hero/east.png")).unwrap();
    let east_img = east_dimg.to_rgba();
    let west_img = east_dimg.clone().fliph().to_rgba();


    Hero {
      pos: pos,
      facing: Facing::South,
      textures: [
        gl_gfx::Texture::from_path(assets::get("hero/north.png")).unwrap(),
        gl_gfx::Texture::from_image(&west_img, &TextureSettings::new()),
        gl_gfx::Texture::from_path(assets::get("hero/south.png")).unwrap(),
        gl_gfx::Texture::from_image(&east_img, &TextureSettings::new()),
      ],
    }
  }

  pub fn walk(&mut self, dir: [f64; 2]) {
    self.pos[0] += dir[0] * SPEED;
    self.pos[1] += dir[1] * SPEED;

    if let Some(f) = Facing::from_dir(dir) {
      self.facing = f;
    }
  }

  pub fn push(&mut self, f: physics::ForceVector) {
    self.pos[0] += f[0];
    self.pos[1] += f[1];
  }

  pub fn tex(&self) -> &gl_gfx::Texture {
    match self.facing {
      Facing::North => &self.textures[0],
      Facing::West => &self.textures[1],
      Facing::South => &self.textures[2],
      Facing::East => &self.textures[3],
    }
  }

  pub fn x(&self) -> f64 { self.pos[0] }
  pub fn y(&self) -> f64 { self.pos[1] }
}

impl physics::Collidable for Hero {
  fn hitbox(&self) -> physics::HitBox {
    [
      self.pos[0] + 0.5 - HBOX_SIZE[0] / 2.0,
      self.pos[1] + 0.5 - HBOX_SIZE[1] / 2.0,
      HBOX_SIZE[0],
      HBOX_SIZE[1],
    ]
  }
}

pub enum Facing {
  North,
  West,
  South,
  East,
}

impl Facing {
  fn from_dir(dir: [f64; 2]) -> Option<Self> {
    let x = dir[0];
    let y = dir[1];
    if x == 0.0 {
      return if y > 0.0 {
        Some(Facing::South)
      } else if y < 0.0 {
        Some(Facing::North)
      } else {
        None
      };
    }

    if y == 0.0 {
      return if x > 0.0 {
        Some(Facing::East)
      } else if x < 0.0 {
        Some(Facing::West)
      } else {
        None
      };
    }

    None
  }
}
