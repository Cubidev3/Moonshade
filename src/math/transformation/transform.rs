use std::ops::{Mul};
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformation::basis::Basis;
use crate::math::transformation::rotation::Rotation;
use crate::math::transformation::scale::Scale;
use crate::math::transformation::transformation::Transformation;
use crate::math::transformation::translation::Translation;
use crate::math::vector::Vector;

#[derive(Copy, Clone)]
pub struct Transform {
    basis: Basis,
    translation: Translation,
    rotation: Rotation,
    scale: Scale
}

impl Transform {
    pub fn new(basis: Basis, translation: Translation, rotation: Rotation, scale: Scale) -> Transform {
        Transform {
            basis,
            translation,
            rotation,
            scale
        }
    }

    pub fn translation(translation: Vector) -> Transform {
        Transform {
            basis: Basis::XYZ,
            translation: Translation::new(translation),
            rotation: Rotation::ZERO,
            scale: Scale::ONE
        }
    }

    pub fn rotation(look: Vector) -> Transform {
        Transform {
            basis: Basis::XYZ,
            translation: Translation::ZERO,
            rotation: Rotation::look_towards(look).unwrap_or(Rotation::ZERO),
            scale: Scale::ONE
        }
    }

    pub fn scale(scale: Vector) -> Transform {
        Transform {
            basis: Basis::XYZ,
            translation: Translation::ZERO,
            rotation: Rotation::ZERO,
            scale: Scale::new(scale)
        }
    }
}

impl Transformation for Transform {
    fn inverse(&self) -> Self {
        Transform {
            basis: self.basis.inverse(),
            translation: self.translation.inverse(),
            rotation: self.rotation.inverse(),
            scale: self.scale.inverse()
        }
    }

    fn matrix(&self) -> Matrix {
        self.basis.inverse().matrix() * self.translation.matrix() * self.rotation.matrix() * self.scale.matrix() * self.basis.matrix()
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
         self.basis.inverse() * (self.translation * (self.rotation * (self.scale * (self.basis * rhs))))
    }
}

impl Mul<Point> for Transform {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let rhs = rhs - Point::ZERO;
        Point::ZERO + self * rhs
    }
}

impl Mul<Ray> for Transform {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray::new(
            self * rhs.origin,
            self * rhs.direction
        )
    }
}