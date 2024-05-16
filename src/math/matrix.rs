use std::ops::Mul;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::vector::Vector;

#[derive(Copy, Clone)]
pub struct Matrix {
    pub a: f64, pub b: f64, pub c: f64, pub d: f64,
    pub e: f64, pub f: f64, pub g: f64, pub h: f64,
    pub i: f64, pub j: f64, pub k: f64, pub l: f64,
    pub m: f64, pub n: f64, pub o: f64, pub p: f64
}

impl Matrix {
    pub const IDENTITY: Matrix = Matrix {
        a: 1.0, b: 0.0, c: 0.0, d: 0.0,
        e: 0.0, f: 1.0, g: 0.0, h: 0.0,
        i: 0.0, j: 0.0, k: 1.0, l: 0.0,
        m: 0.0, n: 0.0, o: 0.0, p: 1.0
    };

    pub fn transposed(self) -> Matrix {
        Matrix {
            a: self.a, b: self.e, c: self.i, d: self.m,
            e: self.b, f: self.f, g: self.j, h: self.n,
            i: self.c, j: self.g, k: self.k, l: self.o,
            m: self.d, n: self.h, o: self.l, p: self.p
        }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.a * rhs.x + self.b * rhs.y + self.c * rhs.z,
            self.e * rhs.x + self.f * rhs.y + self.g * rhs.z,
            self.i * rhs.x + self.j * rhs.y + self.k * rhs.z
        )
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(
            self.a * rhs.x + self.b * rhs.y + self.c * rhs.z + self.d * 1.0,
            self.e * rhs.x + self.f * rhs.y + self.g * rhs.z + self.h * 1.0,
            self.i * rhs.x + self.j * rhs.y + self.k * rhs.z + self.l * 1.0
        )
    }
}

impl Mul<Ray> for Matrix {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray::new(
            self * rhs.origin,
            self * rhs.direction
        )
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix {
            a: self.a * rhs.a + self.b * rhs.e + self.c * rhs.i + self.d * rhs.m, b: self.a * rhs.b + self.b * rhs.f + self.c * rhs.j + self.d * rhs.n, c: self.a * rhs.c + self.b * rhs.g + self.c * rhs.k + self.d * rhs.o, d: self.a * rhs.d + self.b * rhs.h + self.c * rhs.l + self.d * rhs.p,
            e: self.e * rhs.a + self.f * rhs.e + self.g * rhs.i + self.h * rhs.m, f: self.e * rhs.b + self.f * rhs.f + self.g * rhs.j + self.h * rhs.n, g: self.e * rhs.c + self.f * rhs.g + self.g * rhs.k + self.h * rhs.o, h: self.e * rhs.d + self.f * rhs.h + self.g * rhs.l + self.h * rhs.p,
            i: self.i * rhs.a + self.j * rhs.e + self.k * rhs.i + self.l * rhs.m, j: self.i * rhs.b + self.j * rhs.f + self.k * rhs.j + self.l * rhs.n, k: self.i * rhs.c + self.j * rhs.g + self.k * rhs.k + self.l * rhs.o, l: self.i * rhs.d + self.j * rhs.h + self.k * rhs.l + self.l * rhs.p,
            m: self.m * rhs.a + self.n * rhs.e + self.o * rhs.i + self.p * rhs.m, n: self.m * rhs.b + self.n * rhs.f + self.o * rhs.j + self.p * rhs.n, o: self.m * rhs.c + self.n * rhs.g + self.o * rhs.k + self.p * rhs.o, p: self.m * rhs.d + self.n * rhs.h + self.o * rhs.l + self.p * rhs.p,
        }
    }
}