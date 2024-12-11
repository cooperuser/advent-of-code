mod vectormap;
mod vectorset;
#[allow(unused_imports)]
pub use vectormap::Vector3Map;
#[allow(unused_imports)]
pub use vectorset::Vector3Set;

use crate::vector::Vector;

pub const ZERO: Vector3 = Vector3::new(0, 0, 0);

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    #[allow(dead_code)]
    pub const fn new_i32(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
            z: z as i64,
        }
    }

    #[allow(dead_code)]
    pub const fn new_usize(x: usize, y: usize, z: usize) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
            z: z as i64,
        }
    }

    #[allow(dead_code)]
    pub const fn zero() -> Self {
        ZERO
    }

    pub const fn add(a: Vector3, b: Vector3) -> Self {
        Self {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub const fn sub(a: Vector3, b: Vector3) -> Self {
        Self {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }

    #[allow(dead_code)]
    pub fn range(a: Vector3, b: Vector3) -> impl Iterator<Item = Vector3> {
        (a.z..b.z).flat_map(move |z| {
            (a.y..b.y).flat_map(move |y| (a.x..b.x).map(move |x| Vector3::new(x, y, z)))
        })
    }

    #[allow(dead_code)]
    pub fn iter(self) -> impl Iterator<Item = Vector3> {
        (0..self.z).flat_map(move |z| {
            (0..self.y).flat_map(move |y| (0..self.x).map(move |x| Vector3::new(x, y, z)))
        })
    }

    pub const fn normalized(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    pub const fn volume(&self) -> i64 {
        self.x * self.y * self.z
    }

    pub const fn contained_in(&self, a: Vector3, b: Vector3) -> bool {
        let x = self.x >= a.x && self.x < b.x;
        let y = self.y >= a.y && self.y < b.y;
        let z = self.z >= a.z && self.z < b.z;
        x && y && z
    }
}

impl Vector3 {
    pub const fn xy(&self) -> Vector {
        Vector::new(self.x, self.y)
    }

    pub const fn xz(&self) -> Vector {
        Vector::new(self.x, self.z)
    }

    pub const fn yx(&self) -> Vector {
        Vector::new(self.y, self.x)
    }

    pub const fn yz(&self) -> Vector {
        Vector::new(self.y, self.z)
    }

    pub const fn zx(&self) -> Vector {
        Vector::new(self.z, self.x)
    }

    pub const fn zy(&self) -> Vector {
        Vector::new(self.z, self.y)
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::add(self, rhs)
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::sub(self, rhs)
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<i64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<i64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: i64) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Mul<Vector3> for i64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<Vector3> for i64 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        rhs / self
    }
}

impl std::ops::MulAssign<i64> for Vector3 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::DivAssign<i64> for Vector3 {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl From<(i64, i64, i64)> for Vector3 {
    fn from(value: (i64, i64, i64)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl From<(i32, i32, i32)> for Vector3 {
    fn from(value: (i32, i32, i32)) -> Self {
        Self::new(value.0 as i64, value.1 as i64, value.2 as i64)
    }
}

impl From<(usize, usize, usize)> for Vector3 {
    fn from(value: (usize, usize, usize)) -> Self {
        Self::new(value.0 as i64, value.1 as i64, value.2 as i64)
    }
}
