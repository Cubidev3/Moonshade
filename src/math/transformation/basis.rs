use std::ops::{Mul};
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformation::transformation::Transformation;
use crate::math::vector::Vector;

#[derive(Copy, Clone)]
pub struct Basis {
    u: Vector,
    v: Vector,
    w: Vector
}

impl Basis {
    pub const XYZ: Basis = Basis {
        u: Vector::RIGHT,
        v: Vector::UP,
        w: Vector::FORWARD
    };

    pub const YXZ: Basis = Basis {
        u: Vector::UP,
        v: Vector::RIGHT,
        w: Vector::FORWARD
    };

    pub const XZY: Basis = Basis {
        u: Vector::RIGHT,
        v: Vector::FORWARD,
        w: Vector::UP
    };

    pub fn basis_from_up(up: Vector) -> Option<Basis> {
        let v = up.normalized()?;

        let u = match v {
            Vector { x: 0.0, y, z } => Vector::new(0.0, -z, y),
            Vector { x, y: 0.0, z } => Vector::new(-z, 0.0, x),
            Vector { x, y, z } => Vector::new(-y, x, z)
        };

        let w = Vector::cross(u, v);
        Some(Basis::basis_unchecked(u, v, w))
    }

    pub fn basis_from_up_and_right(up: Vector, right: Vector) -> Option<Basis> {
        let v = up.normalized()?;
        let w = Vector::cross(v, right).normalized()?;
        let u = Vector::cross(v, w);
        Some(Basis::basis_unchecked(u, v, w))
    }

    pub fn basis(u: Vector, v: Vector, w: Vector) -> Option<Basis> {
        let determinant = {
            let right = u.x * v.y * w.z + u.y * v.z * w.x + u.z * v.x * w.y;
            let left = u.x * v.z * w.y + u.y * v.x * w.z + u.z * v.y * w.x;

            right - left
        };

        if determinant == 0.0 {
            return None;
        }

        Some(Basis::basis_unchecked(u, v, w))
    }

    pub fn basis_unchecked(u: Vector, v: Vector, w: Vector) -> Basis {
        Basis {
            u, v, w
        }
    }
}

impl Transformation for Basis {
    fn inverse(&self) -> Self {
        Basis::basis_unchecked(
            Vector::new(self.u.x, self.v.x, self.w.x),
            Vector::new(self.u.y, self.v.y, self.w.y),
            Vector::new(self.u.z, self.v.z, self.w.z)
        )
    }

    fn matrix(&self) -> Matrix {
        Matrix {
            a: self.u.x, b: self.u.y, c: self.u.z, d: 0.0,
            e: self.v.x, f: self.v.y, g: self.v.z, h: 0.0,
            i: self.w.x, j: self.w.y, k: self.w.z, l: 0.0,
            m: 0.0,      n: 0.0,      o: 0.0,      p: 1.0
        }
    }
}

impl Mul<Vector> for Basis {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(Vector::dot(rhs, self.u), Vector::dot(rhs, self.v), Vector::dot(rhs, self.w))
    }
}

impl Mul<Point> for Basis {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let rhs = rhs - Point::ZERO;
        Point::ZERO + Vector::new(Vector::dot(rhs, self.u), Vector::dot(rhs, self.v), Vector::dot(rhs, self.w))
    }
}

impl Mul<Ray> for Basis {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray::new(
            self * rhs.origin,
            self * rhs.direction
        )
    }
}