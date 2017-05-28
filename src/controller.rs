pub struct Controller<T: PartialEq> {
  pub dpad: DPad<T>,
}

impl<T: PartialEq> Controller<T> {
  pub fn new(up: T, left: T, down: T, right: T) -> Controller<T> {
    Controller {
      dpad: DPad::new(up, left, down, right),
    }
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
}
