use bevy::{math::Vec3, render::color::Color};
use num_enum::TryFromPrimitive;
use std::{
    convert::TryFrom,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Index,
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn move_up(&mut self, translation: &mut Vec3, map: &Map) {
        if self.y > 0 && !map[self.y - 1][self.x].is_solid() {
            self.y -= 1;
            translation.y += crate::TILE_SIZE;
        }
    }

    pub fn move_down(&mut self, translation: &mut Vec3, map: &Map) {
        if self.y < map.size - 1 && !map[self.y + 1][self.x].is_solid() {
            self.y += 1;
            translation.y -= crate::TILE_SIZE;
        }
    }

    pub fn move_left(&mut self, translation: &mut Vec3, map: &Map) {
        if self.x > 0 && !map[self.y][self.x - 1].is_solid() {
            self.x -= 1;
            translation.x -= crate::TILE_SIZE;
        }
    }

    pub fn move_right(&mut self, translation: &mut Vec3, map: &Map) {
        if self.x < map.size - 1 && !map[self.y][self.x + 1].is_solid() {
            self.x += 1;
            translation.x += crate::TILE_SIZE;
        }
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Tile {
    Empty,
    Block,
    Start,
    Goal,
    Enemy,
}

impl Tile {
    pub fn color(&self) -> Color {
        match self {
            Tile::Block => Color::RED,
            Tile::Start => Color::GREEN,
            Tile::Goal => Color::YELLOW,
            Tile::Empty | Tile::Enemy => Color::WHITE,
        }
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, Tile::Block)
    }
}

#[derive(Debug)]
pub struct Map {
    pub size: usize,
    layout: Vec<Tile>,
}

impl Map {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mut layout = Vec::new();
        let rows: Vec<io::Result<String>> = BufReader::new(file).lines().collect();
        let size = rows.len();
        for row in rows.iter() {
            let mut columns = 0;
            if let Ok(row) = row {
                for value in row.split(',') {
                    columns += 1;
                    layout.push(Tile::try_from(value.parse::<u8>().unwrap()).unwrap());
                }
                assert_eq!(
                    columns, size,
                    "Number of columns must be equal to number of rows"
                );
            }
        }
        Ok(Self { size, layout })
    }
}

impl Index<usize> for Map {
    type Output = [Tile];

    fn index(&self, row: usize) -> &Self::Output {
        let start = self.size * row;
        &self.layout[start..start + self.size]
    }
}
