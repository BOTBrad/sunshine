struct Board {
  width: i32,
  height: i32,
}

struct Hero {
  pos: (i32, i32),
}

fn main() {
  let board = Board {
    width: 20,
    height: 20,
  };

  let hero = Hero {
    pos: (5, 5),
  };

  let mut wall = "".to_string();

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
