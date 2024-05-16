use std::ops::{Mul};
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformation::transformation::Transformation;
use crate::math::vector::Vector;

#[derive(Copy, Clone)]
pub struct Translation {
    displacement: Vector
}

impl Translation {
    pub const ZERO: Translation = Translation {
        displacement: Vector::ZERO
    };

    pub fn new(displacement: Vector) -> Translation {
        Translation { displacement }
    }
}

impl Transformation for Translation {
    fn inverse(&self) -> Self {
        Translation {
            displacement: -self.displacement
        }
    }

    fn matrix(&self) -> Matrix {
        Matrix {
            a: 1.0, b: 0.0, c: 0.0, d: self.displacement.x,
            e: 0.0, f: 1.0, g: 0.0, h: self.displacement.y,
            i: 0.0, j: 0.0, k: 1.0, l: self.displacement.z,
            m: 0.0, n: 0.0, o: 0.0, p: 1.0
        }
    }
}

impl Mul<Vector> for Translation {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs
    }
}

impl Mul<Point> for Translation {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        rhs + self.displacement
    }
}

impl Mul<Ray> for Translation {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray::new(
            self * rhs.origin,
            rhs.direction
        )
    }
}
