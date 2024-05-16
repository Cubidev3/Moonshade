use std::ops::{Mul};
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformation::transformation::Transformation;
use crate::math::vector::Vector;

#[derive(Copy, Clone)]
pub struct Scale {
    ratio: Vector
}

impl Scale {
    pub const ONE: Scale = Scale {
        ratio: Vector::ONE
    };

    pub fn new(ratio: Vector) -> Scale {
        Scale {
            ratio
        }
    }
}

impl Transformation for Scale {
    fn inverse(&self) -> Self {
        Scale {
            ratio: Vector::new(1.0 / self.ratio.x, 1.0 / self.ratio.y, 1.0 / self.ratio.z)
        }
    }

    fn matrix(&self) -> Matrix {
        Matrix {
            a: self.ratio.x, b: 0.0,          c: 0.0,          d: 0.0,
            e: 0.0,          f: self.ratio.y, g: 0.0,          h: 0.0,
            i: 0.0,          j: 0.0,          k: self.ratio.z, l: 0.0,
            m: 0.0,          n: 0.0,          o: 0.0,          p: 1.0
        }
    }
}

impl Mul<Vector> for Scale {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::component_wise_multiplication(rhs, self.ratio)
    }
}

impl Mul<Point> for Scale {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::ZERO + Vector::component_wise_multiplication(rhs - Point::ZERO, self.ratio)
    }
}

impl Mul<Ray> for Scale {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray::new(
            self * rhs.origin,
            self * rhs.direction
        )
    }
}