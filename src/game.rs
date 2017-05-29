use graphics;
use opengl_graphics as gl_gfx;
use piston::input;

use assets;
use controller;
use hero;
use world;

pub struct State {
  hero: hero::Hero,
  world: world::World,
  gl: gl_gfx::GlGraphics,

  hero_tex: gl_gfx::Texture,
}

const WORLD_STR: &str = "\
####################
#                  #
#                  #
#                  #
#  ####            #
#   #              #
#    #             #
#                  #
#                  #
#                  #
#                  #
#                  #
#               #  #
#                  #
#                  #
#                  #
#                  #
#                  #
#                  #
####################";

impl State {
  pub fn new(gl: gl_gfx::GlGraphics) -> State {
    State {
      hero: hero::Hero {
        pos: (10.0, 10.0),
      },
      world: world::World::from_str(20, 20, WORLD_STR),
      gl: gl,

      hero_tex: gl_gfx::Texture::from_path(assets::path().join("hero.png")).unwrap(),
    }
  }

  pub fn draw(&mut self, args: &input::RenderArgs) {
    let hero = &self.hero;
    let world = &self.world;
    let hero_tex = &self.hero_tex;


    self.gl.draw(args.viewport(), |c, g| {
      graphics::clear([0.0, 0.0, 0.0, 1.0], g);
      let (x, y) = hero.pos;

      for x in 0..world.tiles.len() {
        for y in 0..world.tiles[x].len() {
          match world.tiles[x][y] {
            world::Tile::Wall => {
              graphics::rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [(x as f64) * 16.0, (y as f64) * 16.0, 16.0, 16.0],
                c.transform, g);
            },

            world::Tile::Floor => {},
          }
        }
      }

      graphics::Image::new()
        .rect([x * 16.0, y * 16.0, 16.0, 16.0])
        .draw(hero_tex, &graphics::DrawState::default(), c.transform, g);
    });
  }

  pub fn update<T: PartialEq>(&mut self, ctl: &controller::Controller<T>) {
    self.hero.walk(ctl.dpad.flatten());
  }
}
