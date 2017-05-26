struct Board {
  width: i32,
  height: i32,
}

struct Hero {
  pos: (i32, i32),
}

fn view(board: Board, hero: Hero) {
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
  let board = Board {
    width: 20,
    height: 20,
  };

  let hero = Hero {
    pos: (5, 5),
  };

  view(board, hero);
}
