pub enum Dir {
  Right,
  Up,
  Left,
  Down,
}

pub fn to_dir<T: PartialEq>(r: T, u: T, l: T, d: T, input: T) -> Option<Dir> {
  if input == r {
    Some(Dir::Right)
  } else if input == u {
    Some(Dir::Up)
  } else if input == l {
    Some(Dir::Left)
  } else if input == d {
    Some(Dir::Down)
  } else {
    None
  }
}

#[derive(Clone)]
pub struct Hero {
  pub pos: (i32, i32),
}

impl Hero {
  pub fn walk(&mut self, d: Dir) {
    let (x, y) = self.pos;
    match d {
      Dir::Right => self.pos = (x + 1, y),
      Dir::Up => self.pos = (x, y - 1),
      Dir::Left => self.pos = (x - 1, y),
      Dir::Down => self.pos = (x, y + 1),
    }
  }
}
