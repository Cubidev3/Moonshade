use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::vector::Vector;
use crate::surfaces::material::Material;
use crate::surfaces::surface::{Surface, SurfacePoint};

pub struct Sphere {
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub fn new(radius: f64, material: Material) -> Sphere {
        Sphere { radius, material }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: Ray) -> Option<SurfacePoint> {
        let delta: f64 = Vector::dot(ray.origin - Point::ZERO, ray.direction).powi(2)
            + ray.direction.length_squared() * self.radius.powi(2)
            - ray.direction.length_squared() * (ray.origin - Point::ZERO).length_squared();

        if delta < f64::EPSILON {
            return None;
        }

        let t1 = (-Vector::dot(ray.origin - Point::ZERO, ray.direction) - delta.sqrt()) / ray.direction.length_squared();
        let t2 = (-Vector::dot(ray.origin - Point::ZERO, ray.direction) + delta.sqrt()) / ray.direction.length_squared();

        let t = if t1 <= t2 && t1 / ray.direction.length() > 0.00001 { t1 }
            else if t2 / ray.direction.length() > 0.00001 { t2 }
            else { return None };

        let point: Point = ray.point_at(t);
        Some(SurfacePoint {
            t,
            point,
            normal: point - Point::ZERO,
            material: self.material
        })
    }
}