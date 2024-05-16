use std::ops::{Mul};
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformation::basis::Basis;
use crate::math::transformation::transformation::Transformation;
use crate::math::vector::Vector;

#[derive(Copy, Clone)]
pub struct Rotation {
    basis: Basis,
    sine: f64,
    cosine: f64
}

impl Rotation {
    pub const ZERO: Rotation = Rotation {
        basis: Basis::XYZ,
        sine: 0.0,
        cosine: 1.0
    };

    pub fn look_towards(look: Vector) -> Option<Rotation> {
        let look = look.normalized()?;

        let axis = Vector::cross(look, Vector::FORWARD);
        let (sine, cosine) = (axis.length(), look.z);

        let basis = match Basis::basis_from_up(axis) {
            Some(basis) => basis,
            _ => Basis::XYZ
        };

        Some(Rotation {
            basis,
            sine,
            cosine
        })
    }

    pub fn on_x(radians: f64) -> Rotation {
        let (sine, cosine) = radians.sin_cos();
        Rotation {
            basis: Basis::YXZ,
            sine,
            cosine
        }
    }

    pub fn on_y(radians: f64) -> Rotation {
        let (sine, cosine) = radians.sin_cos();
        Rotation {
            basis: Basis::XYZ,
            sine,
            cosine
        }
    }

    pub fn on_z(radians: f64) -> Rotation {
        let (sine, cosine) = radians.sin_cos();
        Rotation {
            basis: Basis::XZY,
            sine,
            cosine
        }
    }
}

impl Transformation for Rotation {
    fn inverse(&self) -> Self {
        Rotation {
            basis: self.basis,
            sine: -self.sine,
            cosine: self.cosine
        }
    }

    fn matrix(&self) -> Matrix {
        let rotate_y = Matrix {
            a: self.cosine, b: 0.0, c: self.sine,   d: 0.0,
            e: 0.0,         f: 1.0, g: 0.0,         h: 0.0,
            i: -self.sine,  j: 0.0, k: self.cosine, l: 0.0,
            m: 0.0,         n: 0.0, o: 0.0,         p: 1.0
        };
        
        self.basis.inverse().matrix() * rotate_y * self.basis.matrix()
    }
}

impl Mul<Vector> for Rotation {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let rhs = self.basis * rhs;
        self.basis.inverse() * Vector::new(
            self.cosine * rhs.x + self.sine * rhs.z,
            rhs.y,
            self.cosine * rhs.z - self.sine * rhs.x
        )
    }
}

impl Mul<Point> for Rotation {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let rhs = self.basis * (rhs - Point::ZERO);
        Point::ZERO + self * rhs
    }
}

impl Mul<Ray> for Rotation {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray::new(
            self * rhs.origin,
            self * rhs.direction
        )
    }
}