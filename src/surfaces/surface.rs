use crate::math::normal::Normal;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::vector::Vector;
use crate::surfaces::material::Material;

pub trait Surface {
    fn intersect(&self, ray: Ray) -> Option<SurfacePoint>;
}

#[derive(Copy, Clone)]
pub struct SurfacePoint {
    pub t: f64,
    pub point: Point,
    pub normal: Vector,
    pub material: Material
}