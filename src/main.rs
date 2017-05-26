struct Board {
  width: i32,
  height: i32,
}

fn main() {
  let board = Board {
    width: 20,
    height: 20,
  };

  let mut wall = "".to_string();

  for _ in 0..(board.width + 2) {
    wall.push('#');
  }

  println!("{}", wall);
  for _ in 0..board.height {
    print!("#");
    for _ in 0..board.width {
      print!(" ");
    }
    print!("#\n");
  }
  println!("{}", wall);
}
