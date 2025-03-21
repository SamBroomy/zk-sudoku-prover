use std::ops::{Index, IndexMut};

use super::{Cell, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point {
    x: Position,
    y: Position,
}

impl Point {
    pub fn new(x: Position, y: Position) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> Position {
        self.x
    }

    pub fn y(&self) -> Position {
        self.y
    }
}

impl Index<Point> for [[Cell; 9]; 9] {
    type Output = Cell;

    fn index(&self, index: Point) -> &Self::Output {
        &self[index.x][index.y]
    }
}
impl IndexMut<Point> for [[Cell; 9]; 9] {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}
