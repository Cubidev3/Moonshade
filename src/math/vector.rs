use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub const ZERO: Vector = Vector { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vector = Vector { x: 1.0, y: 1.0, z: 1.0 };
    pub const RIGHT: Vector = Vector { x: 1.0, y: 0.0, z: 0.0 };
    pub const LEFT: Vector = Vector { x: -1.0, y: 0.0, z: 0.0 };
    pub const UP: Vector = Vector { x: 0.0, y: 1.0, z: 0.0 };
    pub const DOWN: Vector = Vector { x: 0.0, y: -1.0, z: 0.0 };
    pub const FORWARD: Vector = Vector { x: 0.0, y: 0.0, z: 1.0 };
    pub const BACKWARD: Vector = Vector { x: 0.0, y: 0.0, z: -1.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn dot(a: Vector, b: Vector) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn absolute_dot(a: Vector, b: Vector) -> f64 {
        Vector::dot(a, b).abs()
    }

    pub fn cross(a: Vector, b: Vector) -> Vector {
        Vector {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x
        }
    }

    pub fn sine(a: Vector, b: Vector) -> f64 {
        Vector::cross(a.normalized_or_zero(), b.normalized_or_zero()).length()
    }

    pub fn cosine(a: Vector, b: Vector) -> f64 {
        Vector::dot(a.normalized_or_zero(), b.normalized_or_zero())
    }

    pub fn component_wise_multiplication(a: Vector, b: Vector) -> Vector {
        Vector {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z
        }
    }

    pub fn projection(a: Vector, onto: Vector) -> Vector {
        Vector::dot(a, onto) * onto.inverse_or_zero()
    }

    pub fn scalar_projection(a: Vector, onto: Vector) -> f64 {
        Vector::dot(a, onto.normalized_or_zero())
    }

    pub fn rejection(a: Vector, onto: Vector) -> Vector {
        a - Vector::projection(a, onto)
    }

    pub fn scalar_rejection(a: Vector, onto: Vector) -> f64 {
        Self::rejection(a, onto).length()
    }

    pub fn reflection(a: Vector, axis: Vector) -> Vector {
        a - 2.0 * Vector::projection(a, axis)
    }

    pub fn length_squared(self) -> f64 {
        Vector::dot(self, self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Option<Vector> {
        if self.is_approximately_zero() {
            return None;
        }

        Some(self / self.length())
    }

    pub fn normalized_or_zero(self) -> Vector {
        self.normalized().unwrap_or(Vector::ZERO)
    }

    pub fn inverse(self) -> Option<Vector> {
        if self.is_approximately_zero() {
            return None;
        }

        Some(self / self.length_squared())
    }

    pub fn inverse_or_zero(self) -> Vector {
        self.inverse().unwrap_or(Vector::ZERO)
    }

    pub fn absolute(self) -> Vector {
        Vector { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    pub fn sign(self) -> Vector {
        Vector { x: self.x.signum(), y: self.y.signum(), z: self.z.signum() }
    }

    pub fn is_approximately_zero(self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        let factor: f64 = 1.0 / rhs;
        self * factor
    }
}

impl Div<Vector> for f64 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        rhs.inverse_or_zero() * self
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        let factor: f64 = 1.0 / rhs;

        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector ({}, {}, {})", self.x, self.y, self.z)
    }
}