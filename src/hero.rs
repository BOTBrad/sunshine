use logic::physics;

pub struct Hero {
  pub pos: (f64, f64),
}

const SPEED: f64 = 1.0/16.0;

impl Hero {
  pub fn walk(&mut self, dir: (f64, f64)) {
    let (d_x, d_y) = dir;
    let (x, y) = self.pos;

    let v_x = SPEED * (d_x as f64);
    let v_y = SPEED * (d_y as f64);

    self.pos = (x + v_x, y + v_y);
  }

  pub fn push(&mut self, f: physics::ForceVector) {
    let (x, y) = self.pos;
    self.pos = (x + f[0], y + f[1]);
  }
}
