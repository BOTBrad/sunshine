use std::vec::Vec;

use graphics;
use opengl_graphics as gl_gfx;
use piston::input;

use assets;
use controller;
use hero;
use logic::physics;
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
        pos: [10.0, 10.0],
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
        .rect([hero.pos[0] * 16.0, hero.pos[1] * 16.0, 16.0, 16.0])
        .draw(hero_tex, &graphics::DrawState::default(), c.transform, g);
    });
  }

  pub fn update<T: PartialEq>(&mut self, ctl: &controller::Controller<T>) {
    self.hero.walk(ctl.dpad.flatten());
    let tiles = &self.world.tiles;

    // figure out the wall status around us
    let mut walls = [[false, false], [false, false]];
    let x = self.hero.pos[0].floor() as i32;
    let y = self.hero.pos[1].floor() as i32;
    for i in 0..2 {
      if x + i < 0 || x + i >= tiles.len() as i32 {
        continue;
      }

      let col = &tiles[(x + i) as usize];

      for j in 0..2 {
        if y + j < 0 || y + j >= col.len() as i32 {
          continue;
        }

        if let world::Tile::Wall = col[(y + j) as usize] {
          walls[i as usize][j as usize] = true;
        }
      }
    }

    // construct wall hitboxes (overlapping is ok and actually desirable)
    let mut wall_boxes = Vec::<physics::HitBox>::with_capacity(4);
    let xf = x as f64;
    let yf = y as f64;
    if walls[0][0] {
      if walls[1][0] {
        wall_boxes.push([xf, yf, 2.0, 1.0]);
      }

      if walls[0][1] {
        wall_boxes.push([xf, yf, 1.0, 2.0]);
      }

      if !walls[1][0] && !walls[0][1] {
        wall_boxes.push([xf, yf, 1.0, 1.0]);
      }
    }

    if walls[1][1] {
      if walls[1][0] {
        wall_boxes.push([xf + 1.0, yf, 1.0, 2.0]);
      }

      if walls[0][1] {
        wall_boxes.push([xf, yf + 1.0, 2.0, 1.0]);
      }

      if !walls[1][0] && !walls[0][1] {
        wall_boxes.push([xf + 1.0, yf + 1.0, 1.0, 1.0]);
      }
    }

    if !walls[0][0] && !walls[1][1] {
      if walls[1][0] {
        wall_boxes.push([xf + 1.0, yf, 1.0, 1.0]);
      }

      if walls[0][1] {
        wall_boxes.push([xf, yf + 1.0, 1.0, 1.0]);
      }
    }

    // actually collide

    for b in wall_boxes {
      if let Some(f) = physics::collide(&self.hero, &b) {
        self.hero.push(f);
      }
    }
  }
}
