use graphics;
use opengl_graphics as gl_gfx;
use piston::input;

use assets;
use controller;
use hero;

pub struct State {
  hero: hero::Hero,
  gl: gl_gfx::GlGraphics,

  hero_tex: gl_gfx::Texture,
}

impl State {
  pub fn new(gl: gl_gfx::GlGraphics) -> State {
    State {
      hero: hero::Hero {
        pos: (10.0, 10.0),
      },
      gl: gl,

      hero_tex: gl_gfx::Texture::from_path(assets::path().join("hero.png")).unwrap(),
    }
  }

  pub fn draw(&mut self, args: &input::RenderArgs) {
    let hero = &self.hero;
    let hero_tex = &self.hero_tex;


    self.gl.draw(args.viewport(), |c, g| {
      graphics::clear([0.0, 0.0, 0.0, 1.0], g);
      let (x, y) = hero.pos;

      graphics::Image::new()
        .rect([x * 16.0, y * 16.0, 16.0, 16.0])
        .draw(hero_tex, &graphics::DrawState::default(), c.transform, g);
    });
  }

  pub fn update<T: PartialEq>(&mut self, ctl: &controller::Controller<T>) {
    self.hero.walk(ctl.dpad.flatten());
  }
}
