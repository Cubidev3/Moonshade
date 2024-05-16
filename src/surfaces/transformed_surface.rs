use crate::math::matrix::Matrix;
use crate::math::ray::Ray;
use crate::math::transformation::transform::Transform;
use crate::math::transformation::transformation::Transformation;
use crate::surfaces::surface::{Surface, SurfacePoint};

pub struct TransformedSurface<S: Surface> {
    transform_matrix: Matrix,
    inverse_transform: Matrix,
    surface: S
}

impl<S: Surface> TransformedSurface<S> {
    pub fn new(transform: Transform, surface: S) -> TransformedSurface<S> {
        TransformedSurface { transform_matrix: transform.matrix(), inverse_transform: transform.inverse().matrix(), surface }
    }
}

impl<S: Surface> Surface for TransformedSurface<S> {
    fn intersect(&self, ray: Ray) -> Option<SurfacePoint> {
        let transformed_ray = self.inverse_transform * ray;
        let surface_point = self.surface.intersect(transformed_ray)?;

        let real_t = surface_point.t * transformed_ray.direction.length();

        if real_t <= f64::EPSILON {
            return None;
        }

        let surface_point = SurfacePoint {
            t: real_t,
            point: self.transform_matrix * surface_point.point,
            normal: (self.transform_matrix * surface_point.normal),
            material: surface_point.material
        };

        Some(surface_point)
    }
}