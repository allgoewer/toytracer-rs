use std::io;
use std::ops;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    /// Create a new 3-dimensional vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    /// Write the vector to a [`io::Write`]r
    pub fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "{} {} {}", self.0, self.1, self.2)
    }

    /// Return the x component of a vector
    pub fn x(&self) -> f64 {
        self.0
    }

    /// Return the y component of a vector
    pub fn y(&self) -> f64 {
        self.1
    }

    /// Return the x component of a vector
    pub fn z(&self) -> f64 {
        self.2
    }

    /// Return the three components of a vector
    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }

    /// Return the length (magnitude) of a vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Return the squared length (magnitude) of a vector
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// Calculate the dot product of two vectors
    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    /// Calculate the cross product of two vectors
    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    /// Calaculate the unit-vector of self
    pub fn unit(self) -> Self {
        self / self.length()
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Vec3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 *= 1.0 / rhs;
        self.1 *= 1.0 / rhs;
        self.2 *= 1.0 / rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}
