mod vectormap;
mod vectorset;
#[allow(unused_imports)]
pub use vectormap::VectorMap;
#[allow(unused_imports)]
pub use vectorset::VectorSet;

use crate::direction::Direction;

pub const ZERO: Vector = Vector::new(0, 0);
pub const NORTH: Vector = Vector::new(0, -1);
pub const SOUTH: Vector = Vector::new(0, 1);
pub const EAST: Vector = Vector::new(1, 0);
pub const WEST: Vector = Vector::new(-1, 0);

pub const KINGS: [Vector; 8] = [
    Vector::new(-1, -1),
    Vector::new(-1, 0),
    Vector::new(-1, 1),
    Vector::new(0, -1),
    Vector::new(0, 1),
    Vector::new(1, -1),
    Vector::new(1, 0),
    Vector::new(1, 1),
];

pub const KNIGHTS: [Vector; 8] = [
    Vector::new(-2, -1),
    Vector::new(-2, 1),
    Vector::new(-1, -2),
    Vector::new(-1, 2),
    Vector::new(1, -2),
    Vector::new(1, 2),
    Vector::new(2, -1),
    Vector::new(2, 1),
];

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

    #[allow(dead_code)]
    pub const fn zero() -> Self {
        ZERO
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

    #[allow(dead_code)]
    pub fn range(a: Vector, b: Vector) -> impl Iterator<Item = Vector> {
        (a.y..b.y).flat_map(move |y| (a.x..b.x).map(move |x| Vector::new(x, y)))
    }

    #[allow(dead_code)]
    pub fn iter(self) -> impl Iterator<Item = Vector> {
        (0..self.y).flat_map(move |y| (0..self.x).map(move |x| Vector::new(x, y)))
    }

    pub const fn sum(&self) -> i64 {
        self.x + self.y
    }

    pub const fn product(&self) -> i64 {
        self.x * self.y
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

    pub const fn contained_in(&self, a: Vector, b: Vector) -> bool {
        let x = self.x >= a.x && self.x < b.x;
        let y = self.y >= a.y && self.y < b.y;
        x && y
    }

    pub const fn rem_euclid(&self, rhs: Vector) -> Vector {
        Vector::new(self.x.rem_euclid(rhs.x), self.y.rem_euclid(rhs.y))
    }

    pub const fn aabb(a: (Vector, Vector), b: (Vector, Vector)) -> bool {
        let x = a.0.x < b.1.x && a.1.x > b.0.x;
        let y = a.0.y < b.1.y && a.1.y > b.0.y;
        x && y
    }

    pub const fn min(&self, other: Vector) -> Vector {
        let x = if self.x < other.x { self.x } else { other.x };
        let y = if self.y < other.y { self.y } else { other.y };
        Vector::new(x, y)
    }

    pub const fn max(&self, other: Vector) -> Vector {
        let x = if self.x > other.x { self.x } else { other.x };
        let y = if self.y > other.y { self.y } else { other.y };
        Vector::new(x, y)
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

impl std::ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Add<Direction> for Vector {
    type Output = Vector;

    fn add(self, rhs: Direction) -> Self::Output {
        Vector::add(self, rhs.into())
    }
}

impl std::ops::Add<Option<Direction>> for Vector {
    type Output = Vector;

    fn add(self, rhs: Option<Direction>) -> Self::Output {
        match rhs {
            Some(dir) => self + dir,
            None => self,
        }
    }
}

impl std::ops::Sub<Direction> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Direction) -> Self::Output {
        Vector::sub(self, rhs.into())
    }
}

impl std::ops::Sub<Option<Direction>> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Option<Direction>) -> Self::Output {
        match rhs {
            Some(dir) => self - dir,
            None => self,
        }
    }
}

impl std::ops::AddAssign<Direction> for Vector {
    fn add_assign(&mut self, rhs: Direction) {
        let other: Vector = rhs.into();
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::AddAssign<Option<Direction>> for Vector {
    fn add_assign(&mut self, rhs: Option<Direction>) {
        if let Some(dir) = rhs {
            let other: Vector = dir.into();
            self.x += other.x;
            self.y += other.y;
        }
    }
}

impl std::ops::SubAssign<Direction> for Vector {
    fn sub_assign(&mut self, rhs: Direction) {
        let other: Vector = rhs.into();
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::SubAssign<Option<Direction>> for Vector {
    fn sub_assign(&mut self, rhs: Option<Direction>) {
        if let Some(dir) = rhs {
            let other: Vector = dir.into();
            self.x -= other.x;
            self.y -= other.y;
        }
    }
}

impl std::ops::Mul<i64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<i64> for Vector {
    type Output = Vector;

    fn div(self, rhs: i64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::Mul<Vector> for i64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<Vector> for i64 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        rhs / self
    }
}

impl std::ops::MulAssign<i64> for Vector {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::DivAssign<i64> for Vector {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
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
