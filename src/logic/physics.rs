pub fn collide(hurt: HitBox, target: HitBox) -> Option<ForceVector> {
  let h_x = hurt[0];
  let h_y = hurt[1];
  let h_w = hurt[2];
  let h_h = hurt[3];
  let t_x = target[0];
  let t_y = target[1];
  let t_w = target[2];
  let t_h = target[3];

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


pub type HitBox = [f64; 4];
pub type ForceVector = [f64; 2];
