use std::fmt::{Display, Formatter};
use crate::math::point::Point;
use crate::math::vector::Vector;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn from_points(a: Point, b: Point) -> Ray {
        Ray { origin: a, direction: b - a }
    }

    pub fn normalized(&self) -> Option<Ray> {
        Some(Ray { origin: self.origin, direction: self.direction.normalized()? })
    }

    pub fn point_at(self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn t_of_point(self, point: Point) -> Option<f64> {
        if !self.contains(point) {
            return None;
        }

        Some(Vector::dot(self.direction.inverse_or_zero(), point - self.origin))
    }

    pub fn contains(self, point: Point) -> bool {
        Vector::scalar_rejection(self.direction, point - self.origin) == 0.0
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ray: Origin ({}, {}, {}), Direction: ({}, {}, {})", self.origin.x, self.origin.y, self.origin.z, self.direction.x, self.direction.y, self.direction.z)
    }
}