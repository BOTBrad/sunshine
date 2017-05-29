use std::vec::Vec;

pub struct World {
  pub tiles: Vec<Vec<Tile>>,
}

impl World {
  pub fn from_str(width: usize, height: usize, data: &str) -> Self {
    let mut tiles = Vec::with_capacity(width);
    tiles.resize(width, Vec::with_capacity(height));
    for v in tiles.iter_mut() {
      v.resize(height, Tile::Floor);
    }

    let mut i = 0;
    for c in data.chars() {
      if i >= width * height {
        break;
      }

      match c {
        '#' => tiles[i%width][i/width] = Tile::Wall,
        ' ' => tiles[i%width][i/width] = Tile::Floor,
        _ => i -= 1,
      }

      i += 1;
    }

    World {
      tiles: tiles,
    }
  }
}

#[derive(Clone)]
pub enum Tile {
  Wall,
  Floor,
}
