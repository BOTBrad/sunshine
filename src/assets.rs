use std::path::{Path, PathBuf};

pub fn get(p: &str) -> PathBuf {
   Path::new("assets").join(p)
}
