use std::collections::HashMap;

pub struct World {
  tiles: HashMap<i32, HashMap<i32, Tile>>,
}

impl World {
  pub fn new() -> Self {
    World {
      tiles: HashMap::new(),
    }
  }

  pub fn add(&mut self, coords: (i32, i32), tile: Tile) -> &mut Self {
    let (x, y) = coords;
    if self.tiles.contains_key(&x) {
        self.tiles.get_mut(&x).unwrap().insert(y, tile);
    } else {
      self.tiles.insert(x, [(y, tile)].iter().cloned().collect());
    }

    self
  }

  pub fn get(&self, x: i32, y: i32) -> Tile {
    match self.tiles.get(&x).and_then(|y_map| y_map.get(&y)) {
      Some(tile) => *tile,
      None => Tile::Floor,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Tile {
  Wall,
  Floor,
}
