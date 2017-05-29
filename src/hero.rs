use logic::physics;

pub struct Hero {
  pub pos: [f64; 2],
}

const SPEED: f64 = 1.0/16.0;
const HBOX_SIZE: [f64; 2] = [0.75, 0.9];

impl Hero {
  pub fn walk(&mut self, dir: [f64; 2]) {
    self.pos[0] += dir[0] * SPEED;
    self.pos[1] += dir[1] * SPEED;
  }

  pub fn push(&mut self, f: physics::ForceVector) {
    self.pos[0] += f[0];
    self.pos[1] += f[1];
  }

  pub fn hitbox(&self) -> physics::HitBox {
    [
      self.pos[0] + 0.5 - HBOX_SIZE[0] / 2.0,
      self.pos[1] + 0.5 - HBOX_SIZE[1] / 2.0,
      HBOX_SIZE[0],
      HBOX_SIZE[1],
    ]
  }
}
