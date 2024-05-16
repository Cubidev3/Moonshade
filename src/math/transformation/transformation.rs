use std::ops::Mul;
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::vector::Vector;

pub trait Transformation : Mul<Vector> + Mul<Point> + Mul<Ray> {
    fn inverse(&self) -> Self;
    fn matrix(&self) -> Matrix;
}