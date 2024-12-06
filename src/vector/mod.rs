mod vectorset;
#[allow(unused_imports)]
pub use vectorset::VectorSet;

use crate::direction::Direction;

pub const NORTH: Vector = Vector::new(0, -1);
pub const SOUTH: Vector = Vector::new(0, 1);
pub const EAST: Vector = Vector::new(1, 0);
pub const WEST: Vector = Vector::new(-1, 0);

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Vector {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub const fn new_i32(x: i32, y: i32) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }

    #[allow(dead_code)]
    pub const fn new_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }

    pub const fn add(a: Vector, b: Vector) -> Self {
        Self {
            x: a.x + b.x,
            y: a.y + b.y,
        }
    }

    pub const fn sub(a: Vector, b: Vector) -> Self {
        Self {
            x: a.x - b.x,
            y: a.y - b.y,
        }
    }

    pub const fn normalized(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    pub const fn area(&self) -> i64 {
        self.x * self.y
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::add(self, rhs)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::sub(self, rhs)
    }
}

impl std::ops::Add<Direction> for Vector {
    type Output = Vector;

    fn add(self, rhs: Direction) -> Self::Output {
        Vector::add(self, rhs.into())
    }
}

impl std::ops::Sub<Direction> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Direction) -> Self::Output {
        Vector::sub(self, rhs.into())
    }
}

impl From<Direction> for Vector {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => NORTH,
            Direction::South => SOUTH,
            Direction::East => EAST,
            Direction::West => WEST,
        }
    }
}

impl From<(i64, i64)> for Vector {
    fn from(value: (i64, i64)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(i32, i32)> for Vector {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0 as i64, value.1 as i64)
    }
}

impl From<(usize, usize)> for Vector {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as i64, value.1 as i64)
    }
}
