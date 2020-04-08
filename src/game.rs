use std::vec::Vec;

use graphics;
use opengl_graphics as gl_gfx;
use piston::input;

use crate::controller;
use crate::hero;
use crate::logic::physics;
use crate::world;

pub struct State {
  hero: hero::Hero,
  world: world::World,
  gl: gl_gfx::GlGraphics,
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
    let world = WORLD_STR
      .split('\n')
      .enumerate()
      .flat_map(|(y, x_list)|
        x_list
          .chars()
          .enumerate()
          .map(move |(x, t)| ((x as i32, y as i32), match t {
            '#' => world::Tile::Wall,
            _ => world::Tile::Floor,
          }))
      )
      .fold(world::World::new(), |mut w, (coords, tile)| {w.add(coords, tile); w});

    State {
      hero: hero::Hero::new([10.0, 10.0]),
      world: world,
      gl: gl,
    }
  }

  pub fn draw(&mut self, args: &input::RenderArgs) {
    let hero = &self.hero;
    let world = &self.world;

    let x_offset = hero.x() - 10.0;
    let y_offset = hero.y() - 10.0;

    self.gl.draw(args.viewport(), |c, g| {
      graphics::clear([1.0, 1.0, 1.0, 1.0], g);
      for x in 0..20 {
        for y in 0..20 {
          match world.get(x, y) {
            world::Tile::Wall => {
              graphics::rectangle(
                [0.0, 0.0, 0.0, 1.0],
                [(x as f64 - x_offset) * 16.0, (y as f64 - y_offset) * 16.0, 16.0, 16.0],
                c.transform, g);
            },

            world::Tile::Floor => {},
          }
        }
      }

      graphics::Image::new()
        .rect([10.0 * 16.0, 10.0 * 16.0, 16.0, 16.0])
        .draw(hero.tex(), &graphics::DrawState::default(), c.transform, g);
    });
  }

  pub fn update<T: PartialEq>(&mut self, ctl: &controller::Controller<T>) {
    self.hero.walk(ctl.dpad.flatten());
    let world = &self.world;

    // figure out the wall status around us
    let mut walls = [[false, false], [false, false]];
    let x = self.hero.x().floor() as i32;
    let y = self.hero.y().floor() as i32;
    for i in 0..2 {
      for j in 0..2 {
        match world.get(x + i, y + j) {
          world::Tile::Wall => { walls[i as usize][j as usize] = true; }
          _ => {}
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
