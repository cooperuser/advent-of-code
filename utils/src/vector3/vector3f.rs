pub const ZERO: Vector3f = Vector3f::new(0.0, 0.0, 0.0);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3f {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[allow(dead_code)]
    pub const fn new_i32(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    #[allow(dead_code)]
    pub const fn new_usize(x: usize, y: usize, z: usize) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    #[allow(dead_code)]
    pub const fn zero() -> Self {
        ZERO
    }

    pub fn add(a: Vector3f, b: Vector3f) -> Self {
        Self {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub fn sub(a: Vector3f, b: Vector3f) -> Self {
        Self {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }

    pub fn sqr_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        *self / magnitude
    }

    pub fn volume(&self) -> f64 {
        self.x * self.y * self.z
    }

    pub fn contained_in(&self, a: Vector3f, b: Vector3f) -> bool {
        let x = self.x >= a.x && self.x <= b.x;
        let y = self.y >= a.y && self.y <= b.y;
        let z = self.z >= a.z && self.z <= b.z;
        x && y && z
    }

    pub fn rem_euclid(&self, rhs: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x.rem_euclid(rhs.x),
            self.y.rem_euclid(rhs.y),
            self.z.rem_euclid(rhs.z),
        )
    }

    pub fn aabb(a: (Vector3f, Vector3f), b: (Vector3f, Vector3f)) -> bool {
        let x = a.0.x < b.1.x && a.1.x > b.0.x;
        let y = a.0.y < b.1.y && a.1.y > b.0.y;
        let z = a.0.z < b.1.z && a.1.z > b.0.z;
        x && y && z
    }

    /// Find the closest pair of points on two lines defined by a point and a vector.
    /// Given as the value each ray will need to be scaled by and added to the position.
    ///
    /// a: (position, direction)
    /// b: (position, direction)
    pub fn closest_points(a: (Vector3f, Vector3f), b: (Vector3f, Vector3f)) -> Option<(f64, f64)> {
        let a_mag = a.1 * a.1;
        let b_mag = b.1 * b.1;
        if a_mag == 0.0 || b_mag == 0.0 || (a.1 * b.1).powi(2) == a_mag * b_mag {
            return None;
        }
        let c = b.0 - a.0;
        let frac_a = ((a.1 * c) * (b.1 * b.1) - (a.1 * b.1) * (b.1 * c))
            / ((a.1 * a.1) * (b.1 * b.1) - (a.1 * b.1) * (a.1 * b.1));
        let frac_b = ((a.1 * b.1) * (a.1 * c) - (b.1 * c) * (a.1 * a.1))
            / ((a.1 * a.1) * (b.1 * b.1) - (a.1 * b.1) * (a.1 * b.1));
        Some((frac_a, frac_b))
    }

    pub fn are_parallel(a: (Vector3f, Vector3f), b: (Vector3f, Vector3f)) -> bool {
        let a_mag = a.1 * a.1;
        let b_mag = b.1 * b.1;
        a_mag == 0.0 || b_mag == 0.0 || (a.1 * b.1).powi(2) == a_mag * b_mag
    }

    pub fn separated_by(
        a: (Vector3f, Vector3f),
        b: (Vector3f, Vector3f),
        v: Vector3f,
    ) -> Option<(f64, f64, f64)> {
        // a_0_x + a*a_1_x + t*v_x = b_0_x + b*b_1_x
        // a_0_y + a*a_1_y + t*v_y = b_0_y + b*b_1_y
        // a_0_z + a*a_1_z + t*v_z = b_0_z + b*b_1_z
        let t_numerator =
            -(a.0.x * a.1.y * b.1.z) + (a.0.x * a.1.z * b.1.y) + (a.0.y * a.1.x * b.1.z)
                - (a.0.y * a.1.z * b.1.x)
                - (a.0.z * a.1.x * b.1.y)
                + (a.0.z * a.1.y * b.1.x)
                - (a.1.x * b.0.y * b.1.z)
                + (a.1.x * b.0.z * b.1.y)
                + (a.1.y * b.0.x * b.1.z)
                - (a.1.y * b.0.z * b.1.x)
                - (a.1.z * b.0.x * b.1.y)
                + (a.1.z * b.0.y * b.1.x);
        let a_numerator = -(a.0.x * b.1.y * v.z) + (a.0.x * b.1.z * v.y) + (a.0.y * b.1.x * v.z)
            - (a.0.y * b.1.z * v.x)
            - (a.0.z * b.1.x * v.y)
            + (a.0.z * b.1.y * v.x)
            + (b.0.x * b.1.y * v.z)
            - (b.0.x * b.1.z * v.y)
            - (b.0.y * b.1.x * v.z)
            + (b.0.y * b.1.z * v.x)
            + (b.0.z * b.1.x * v.y)
            - (b.0.z * b.1.y * v.x);
        let b_numerator = -(a.0.x * a.1.y * v.z) + (a.0.x * a.1.z * v.y) + (a.0.y * a.1.x * v.z)
            - (a.0.y * a.1.z * v.x)
            - (a.0.z * a.1.x * v.y)
            + (a.0.z * a.1.y * v.x)
            - (a.1.x * b.0.y * v.z)
            + (a.1.x * b.0.z * v.y)
            + (a.1.y * b.0.x * v.z)
            - (a.1.y * b.0.z * v.x)
            - (a.1.z * b.0.x * v.y)
            + (a.1.z * b.0.y * v.x);
        let denominator = (a.1.x * b.1.y * v.z) - (a.1.x * b.1.z * v.y) - (a.1.y * b.1.x * v.z)
            + (a.1.y * b.1.z * v.x)
            + (a.1.z * b.1.x * v.y)
            - (a.1.z * b.1.y * v.x);
        if denominator != 0.0 {
            Some((
                t_numerator / denominator,
                a_numerator / denominator,
                b_numerator / denominator,
            ))
        } else {
            None
        }
    }
}

impl std::ops::Add<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn add(self, rhs: Vector3f) -> Self::Output {
        Vector3f::add(self, rhs)
    }
}

impl std::ops::Sub<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, rhs: Vector3f) -> Self::Output {
        Vector3f::sub(self, rhs)
    }
}

impl std::ops::AddAssign<Vector3f> for Vector3f {
    fn add_assign(&mut self, rhs: Vector3f) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::SubAssign<Vector3f> for Vector3f {
    fn sub_assign(&mut self, rhs: Vector3f) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<Vector3f> for Vector3f {
    type Output = f64;

    fn mul(self, rhs: Vector3f) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl std::ops::Mul<f64> for Vector3f {
    type Output = Vector3f;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f64> for Vector3f {
    type Output = Vector3f;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3f::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Mul<Vector3f> for f64 {
    type Output = Vector3f;

    fn mul(self, rhs: Vector3f) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<Vector3f> for f64 {
    type Output = Vector3f;

    fn div(self, rhs: Vector3f) -> Self::Output {
        rhs / self
    }
}

impl std::ops::MulAssign<f64> for Vector3f {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::DivAssign<f64> for Vector3f {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl From<(f64, f64, f64)> for Vector3f {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl From<(i32, i32, i32)> for Vector3f {
    fn from(value: (i32, i32, i32)) -> Self {
        Self::new(value.0 as f64, value.1 as f64, value.2 as f64)
    }
}

impl From<(usize, usize, usize)> for Vector3f {
    fn from(value: (usize, usize, usize)) -> Self {
        Self::new(value.0 as f64, value.1 as f64, value.2 as f64)
    }
}
