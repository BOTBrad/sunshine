use std::io;

use board;
use hero;

pub struct State {
  board: board::Board,
  hero: hero::Hero,
}

impl State {
  pub fn new() -> State {
    State {
      board: board::Board {
        width: 20,
        height: 20,
      },
      hero: hero::Hero {
        pos: (5, 5),
      },
    }
  }

  pub fn draw(&self) {
    let board = &self.board;
    let hero = &self.hero;

    let mut wall = String::new();
    for _ in 0..(board.width + 2) {
      wall.push('#');
    }

    println!("{}", wall);
    for y in 0..board.height {
      print!("#");
      for x in 0..board.width {
        if (x, y) == hero.pos {
          print!("@");
        } else {
          print!(" ");
        }
      }
      print!("#\n");
    }
    println!("{}", wall);
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
