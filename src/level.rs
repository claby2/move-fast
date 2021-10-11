use crate::map::Map;
use std::{ffi::OsStr, fs, io, path::PathBuf, slice::Iter};

#[derive(Debug)]
pub struct LevelManager {
    paths: Vec<PathBuf>,
    current: usize,
}

impl LevelManager {
    pub fn fetch() -> io::Result<Self> {
        let mut paths = Vec::new();
        let mut level_path = PathBuf::from("assets");
        level_path.push("levels");
        if level_path.exists() {
            for level in fs::read_dir(level_path)?.flatten() {
                let path = level.path();
                if path.extension() == Some(OsStr::new("csv")) {
                    paths.push(path);
                }
            }
            paths.sort();
        }
        Ok(LevelManager { paths, current: 0 })
    }

    pub fn set_level(&mut self, index: usize) {
        self.current = index;
    }

    pub fn iter(&self) -> Iter<'_, PathBuf> {
        self.paths.iter()
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn load(&self) -> io::Result<Map> {
        Map::load(&self.paths[self.current])
    }
}
