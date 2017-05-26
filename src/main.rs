mod board;
mod hero;

use std::io;

fn view(board: &board::Board, hero: &hero::Hero) {
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

fn main() {
  let board = board::Board {
    width: 20,
    height: 20,
  };

  let hero = hero::Hero {
    pos: (5, 5),
  };

  let mut input = String::new();
  loop {
    view(&board, &hero);
    io::stdin().read_line(&mut input).expect("stdin failed");
    input.pop(); // remove the '\n' from the end

    if input == "exit" {
      break;
    }
  }
}
