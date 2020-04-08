pub fn collide(hurt: &dyn Collidable, target: &dyn Collidable) -> Option<ForceVector> {
  let h_box = hurt.hitbox();
  let t_box = target.hitbox();

  let h_x = h_box[0];
  let h_y = h_box[1];
  let h_w = h_box[2];
  let h_h = h_box[3];
  let t_x = t_box[0];
  let t_y = t_box[1];
  let t_w = t_box[2];
  let t_h = t_box[3];

  // get horizontal overlap
  let x = if h_x + h_w > t_x + t_w {
    (t_x + t_w - h_x).max(0.0)
  } else {
    (t_x - (h_x + h_w)).min(0.0)
  };

  let y = if h_y + h_h > t_y + t_h {
    (t_y + t_h - h_y).max(0.0)
  } else {
    (t_y - (h_y + h_h)).min(0.0)
  };

  if x == 0.0 && y == 0.0 {
    return None;
  }

  if x.abs() < y.abs() {
    Some([x, 0.0])
  } else {
    Some([0.0, y])
  }
}

pub trait Collidable {
  fn hitbox(&self) -> HitBox;
}

pub type HitBox = [f64; 4];
pub type ForceVector = [f64; 2];

impl Collidable for HitBox {
  fn hitbox(&self) -> HitBox {
    *self
  }
}
