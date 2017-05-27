use std::io;

use board;
use gfx;
use hero;
use tile;

pub struct State {
  board: board::Board,
  hero: hero::Hero,
}

impl State {
  pub fn new(width: u32, height: u32) -> State {
    State {
      board: board::Board {
        width: width,
        height: height,
      },
      hero: hero::Hero {
        pos: ((width as i32)/2, (height as i32)/2),
      },
    }
  }

  pub fn draw(&self, gfx: &mut gfx::Graphics) {
    let board = &self.board;
    let hero = &self.hero;

    // console output

    let mut wall = String::new();
    for _ in 0..(board.width + 2) {
      wall.push('#');
    }

    println!("{}", wall);
    for y in 0..board.height {
      print!("#");
      for x in 0..board.width {
        if (x as i32, y as i32) == hero.pos {
          print!("@");
        } else {
          print!(" ");
        }
      }
      print!("#\n");
    }
    println!("{}", wall);

    // gfx output

    gfx.clear();

    for x in 0..(board.width + 2) {
      gfx.draw(x, 0, tile::Type::Wall);
      gfx.draw(x, self.board.height + 1, tile::Type::Wall);
    }

    for y in 0..board.height {
      gfx.draw(0, y + 1, tile::Type::Wall);
      gfx.draw(self.board.width + 1, y + 1, tile::Type::Wall);
      for x in 0..board.width {
        let t = if (x as i32, y as i32) == hero.pos {
          tile::Type::Hero
        } else {
          tile::Type::Floor
        };
        gfx.draw(x + 1, y + 1, t);
      }
    }

    gfx.present();
  }

  pub fn update(&mut self) -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin failed");
    input.pop(); // remove the '\n' from the end

    if input == "exit" {
      return true;
    } else {
      let dir = hero::to_dir("d", "w", "a", "s", input.as_str());
      if let Some(d) = dir {
        self.hero.walk(d);
      }
    }
    false
  }
}
