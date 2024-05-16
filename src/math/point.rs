use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::math::vector::Vector;

#[derive(Copy, Clone, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn reflected(self, axis: Vector) -> Option<Point> {
        if self.is_approximately_zero() {
            return None;
        }

        Some(self - 2.0 * Vector::dot(self - Point::ZERO, axis) * axis)
    }

    pub fn distance(a: Point, b: Point) -> f64 {
        (b - a).length()
    }

    pub fn distance_squared(a: Point, b: Point) -> f64 {
        (b - a).length_squared()
    }

    pub fn is_approximately_zero(self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        let factor: f64 = 1.0 / rhs;
        self * factor
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        let factor: f64 = 1.0 / rhs;

        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point ({}, {}, {})", self.x, self.y, self.z)
    }
}