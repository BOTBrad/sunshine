pub struct Controller<T: PartialEq> {
  pub dpad: DPad<T>,
}

impl<T: PartialEq> Controller<T> {
  pub fn new(up: T, left: T, down: T, right: T) -> Controller<T> {
    Controller {
      dpad: DPad::new(up, left, down, right),
    }
  }

  pub fn down(&mut self, key: &T) {
    self.dpad.down(key);
  }

  pub fn up(&mut self, key: &T) {
    self.dpad.up(key);
  }
}

pub struct DPad<T: PartialEq> {
  bindings: [T; 4],
  state: [bool; 4],
}

impl<T: PartialEq> DPad<T> {
  pub fn new(up: T, left: T, down: T, right: T) -> DPad<T> {
    DPad {
      bindings: [up, left, down, right],
      state: [false; 4],
    }
  }

  pub fn down(&mut self, key: &T) {
    for (idx, bind) in self.bindings.iter().enumerate() {
      if key == bind {
        self.state[idx] = true;
      }
    }
  }

  pub fn up(&mut self, key: &T) {
    for (idx, bind) in self.bindings.iter().enumerate() {
      if key == bind {
        self.state[idx] = false;
      }
    }
  }

  pub fn flatten(&self) -> (f64, f64) {
    let lr = (self.state[3] as i8) - (self.state[1] as i8);
    let ud = (self.state[2] as i8) - (self.state[0] as i8);

    let x = lr as f64;
    let y = ud as f64;

    let z = if lr == 0 && ud == 0 {
      1.0
    } else {
      (x.powi(2) + y.powi(2)).sqrt()
    };

    (x / z, y / z)
  }
}
