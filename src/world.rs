use std::collections::HashMap;
use std::vec::Vec;

pub struct World {
  tiles: HashMap<i32, HashMap<i32, Tile>>,
}

impl World {
  pub fn new(tiles: Vec<((i32, i32), Tile)>) -> Self {
    let mut tile_map: HashMap<i32, HashMap<i32, Tile>> = HashMap::new();

    for ((x, y), tile) in tiles {
      if tile_map.contains_key(&x) {
          tile_map.get_mut(&x).unwrap().insert(y, tile);
      } else {
        tile_map.insert(x, [(y, tile)].iter().cloned().collect());
      }
    }

    World {
      tiles: tile_map,
    }
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
